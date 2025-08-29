use std::collections::HashMap;
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::Bearer;
use sea_orm::{DatabaseConnection, Statement, FromQueryResult};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::modules::jwt::Claims;

// Chaves para metadados
pub const ACCESS_GROUP_KEY: &str = "access_groups";
pub const IS_PUBLIC_KEY: &str = "is_public";

// Enum para grupos de acesso
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessGroupEnum {
    VIEWER = 1,
    PREMIUM = 2,
    ADMIN = 3,
    SUPER_ADMIN = 4,
}

impl From<i32> for AccessGroupEnum {
    fn from(value: i32) -> Self {
        match value {
            1 => AccessGroupEnum::VIEWER,
            2 => AccessGroupEnum::PREMIUM,
            3 => AccessGroupEnum::ADMIN,
            4 => AccessGroupEnum::SUPER_ADMIN,
            _ => AccessGroupEnum::VIEWER,
        }
    }
}

impl From<AccessGroupEnum> for i32 {
    fn from(value: AccessGroupEnum) -> Self {
        value as i32
    }
}

// Estrutura para usuário com grupos de acesso
#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct UserWithAccess {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub access_groups: Vec<AccessGroupEnum>,
}

// Estrutura para metadados de rota
#[derive(Debug, Clone)]
pub struct RouteMetadata {
    pub is_public: bool,
    pub required_groups: Vec<AccessGroupEnum>,
}

// Decorator para marcar rota como pública
pub fn Public() -> RouteMetadata {
    RouteMetadata {
        is_public: true,
        required_groups: vec![],
    }
}

// Decorator para requerer grupos de acesso
pub fn RequireAccess(groups: &[AccessGroupEnum]) -> RouteMetadata {
    RouteMetadata {
        is_public: false,
        required_groups: groups.to_vec(),
    }
}

// Middleware de autenticação e autorização
pub struct AuthGuard {
    db: DatabaseConnection,
    jwt_manager: crate::modules::jwt::JwtManager,
}

impl AuthGuard {
    pub fn new(db: DatabaseConnection, jwt_manager: crate::modules::jwt::JwtManager) -> Self {
        Self { db, jwt_manager }
    }

    pub async fn authenticate_and_authorize(
        &self,
        request: &Request,
        metadata: &RouteMetadata,
    ) -> Result<UserWithAccess, StatusCode> {
        // Se a rota é pública, não precisa de autenticação
        if metadata.is_public {
            return Err(StatusCode::UNAUTHORIZED); // Mas ainda precisamos de um usuário válido
        }

        // Extrair token do header Authorization
        let auth_header = request.headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..]; // Remove "Bearer "

        // Validar JWT
        let claims = self.jwt_manager
            .validate_access_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Buscar usuário com grupos de acesso
        let user = self.get_user_with_access_groups(&claims.sub).await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user = user.ok_or(StatusCode::UNAUTHORIZED)?;

        // Verificar se o usuário tem os grupos de acesso necessários
        if !metadata.required_groups.is_empty() {
            let has_access = metadata.required_groups.iter()
                .any(|required_group| {
                    user.access_groups.contains(required_group)
                });

            if !has_access {
                return Err(StatusCode::FORBIDDEN);
            }
        }

        Ok(user)
    }

    async fn get_user_with_access_groups(&self, user_id: &str) -> Result<Option<UserWithAccess>, sea_orm::DbErr> {
        let sql = r#"
            SELECT 
                u.id, u.email, u.name, u.role,
                STRING_AGG(CAST(uag.access_group_id AS VARCHAR), ',') as access_groups
            FROM users u
            LEFT JOIN users_access_groups uag ON u.id = uag.user_id
            WHERE u.id = @P1
            GROUP BY u.id, u.email, u.name, u.role
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![user_id.into()],
        );

        let result = self.db.query_one(stmt).await?;
        
        if let Some(row) = result {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let email: String = row.try_get("", "email").unwrap_or_default();
            let name: String = row.try_get("", "name").unwrap_or_default();
            let role: String = row.try_get("", "role").unwrap_or_default();
            let access_groups_str: Option<String> = row.try_get("", "access_groups").ok();

            let access_groups = if let Some(groups_str) = access_groups_str {
                groups_str.split(',')
                    .filter_map(|s| s.parse::<i32>().ok().map(AccessGroupEnum::from))
                    .collect()
            } else {
                vec![]
            };

            Ok(Some(UserWithAccess {
                id,
                email,
                name,
                role,
                access_groups,
            }))
        } else {
            Ok(None)
        }
    }
}

// Middleware para aplicar autenticação
pub async fn auth_middleware(
    State(db): State<DatabaseConnection>,
    State(jwt_manager): State<crate::modules::jwt::JwtManager>,
    State(route_metadata): State<RouteMetadata>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_guard = AuthGuard::new(db, jwt_manager);
    
    match auth_guard.authenticate_and_authorize(&request, &route_metadata).await {
        Ok(user) => {
            // Adicionar usuário ao request extensions
            request.extensions_mut().insert(user);
            Ok(next.run(request).await)
        }
        Err(status) => {
            let error_response = serde_json::json!({
                "error": "Unauthorized",
                "message": "Acesso negado"
            });
            
            let response = Response::builder()
                .status(status)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(serde_json::to_string(&error_response).unwrap()))
                .unwrap();
            
            Ok(response)
        }
    }
}
