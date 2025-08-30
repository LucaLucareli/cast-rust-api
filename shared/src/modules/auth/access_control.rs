use axum::http::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::enums::access_group_enum::AccessGroupEnum;
use crate::modules::auth::jwt::Claims;
use crate::modules::database::repositories::users_repository::UsersRepository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithAccess {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub access_groups: Vec<AccessGroupEnum>,
}

#[derive(Debug, Clone)]
pub struct RouteMetadata {
    pub is_public: bool,
    pub required_groups: Vec<AccessGroupEnum>,
}

pub fn public() -> RouteMetadata {
    RouteMetadata {
        is_public: true,
        required_groups: vec![],
    }
}

pub fn require_access(groups: &[AccessGroupEnum]) -> RouteMetadata {
    RouteMetadata {
        is_public: false,
        required_groups: groups.to_vec(),
    }
}

pub struct AuthGuard {
    users_repo: UsersRepository,
    jwt_manager: crate::modules::auth::jwt::JwtManager,
}

impl AuthGuard {
    pub fn new(db: DatabaseConnection, jwt_manager: crate::modules::auth::jwt::JwtManager) -> Self {
        Self {
            users_repo: UsersRepository::new(db),
            jwt_manager,
        }
    }

    pub async fn authenticate_and_authorize(
        &self,
        request: &axum::extract::Request,
        metadata: &RouteMetadata,
    ) -> Result<UserWithAccess, StatusCode> {
        // Rotas públicas não precisam de autenticação
        if metadata.is_public {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let auth_header = request
            .headers()
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..];

        // Validar JWT
        let claims: Claims = self
            .jwt_manager
            .validate_access_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Buscar usuário com grupos de acesso usando repository
        let user_model = self
            .users_repo
            .find_by_id(&claims.sub)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let access_groups = self
            .users_repo
            .find_access_groups(&user_model.id)
            .await
            .unwrap_or_default();

        let user = UserWithAccess {
            id: user_model.id.clone(),
            email: user_model.email.clone(),
            name: user_model.name.clone(),
            role: user_model.role.clone(),
            access_groups,
        };

        // Verificar acesso
        if !metadata.required_groups.is_empty() {
            let has_access = metadata
                .required_groups
                .iter()
                .any(|group| user.access_groups.contains(group));

            if !has_access {
                return Err(StatusCode::FORBIDDEN);
            }
        }

        Ok(user)
    }
}
