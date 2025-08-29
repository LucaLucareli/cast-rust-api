use sea_orm::{DatabaseConnection, Statement, FromQueryResult, EntityTrait, QueryFilter, ColumnTrait, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

pub struct UsersRepository {
    db: DatabaseConnection,
}

impl UsersRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, request: CreateUserRequest) -> Result<User, sea_orm::DbErr> {
        let user_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let role = request.role.unwrap_or_else(|| "Viewer".to_string());
        
        // Hash da senha
        let password_hash = hash(request.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| sea_orm::DbErr::Custom(format!("Erro ao hash da senha: {}", e)))?;

        let sql = r#"
            INSERT INTO users (id, email, name, role, password_hash, created_at, updated_at)
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7)
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![
                user_id.into(),
                request.email.into(),
                request.name.into(),
                role.into(),
                password_hash.into(),
                now.into(),
                now.into(),
            ],
        );

        self.db.execute(stmt).await?;

        // Buscar usuário criado
        self.find_by_id(&user_id).await
            .and_then(|user| user.ok_or(sea_orm::DbErr::Custom("Usuário não encontrado após criação".to_string())))
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<User>, sea_orm::DbErr> {
        let sql = r#"
            SELECT id, email, name, role, password_hash, created_at, updated_at
            FROM users
            WHERE id = @P1
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
            let password_hash: String = row.try_get("", "password_hash").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            Ok(Some(User {
                id,
                email,
                name,
                role,
                password_hash,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sea_orm::DbErr> {
        let sql = r#"
            SELECT id, email, name, role, password_hash, created_at, updated_at
            FROM users
            WHERE email = @P1
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![email.into()],
        );

        let result = self.db.query_one(stmt).await?;
        
        if let Some(row) = result {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let email: String = row.try_get("", "email").unwrap_or_default();
            let name: String = row.try_get("", "name").unwrap_or_default();
            let role: String = row.try_get("", "role").unwrap_or_default();
            let password_hash: String = row.try_get("", "password_hash").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            Ok(Some(User {
                id,
                email,
                name,
                role,
                password_hash,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all(&self, limit: Option<u64>, offset: Option<u64>) -> Result<Vec<User>, sea_orm::DbErr> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let sql = r#"
            SELECT id, email, name, role, password_hash, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            OFFSET @P1 ROWS
            FETCH NEXT @P2 ROWS ONLY
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![offset.into(), limit.into()],
        );

        let results = self.db.query_all(stmt).await?;
        
        let mut users = Vec::new();
        for row in results {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let email: String = row.try_get("", "email").unwrap_or_default();
            let name: String = row.try_get("", "name").unwrap_or_default();
            let role: String = row.try_get("", "role").unwrap_or_default();
            let password_hash: String = row.try_get("", "password_hash").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            users.push(User {
                id,
                email,
                name,
                role,
                password_hash,
                created_at,
                updated_at,
            });
        }

        Ok(users)
    }

    pub async fn update(&self, user_id: &str, request: UpdateUserRequest) -> Result<Option<User>, sea_orm::DbErr> {
        let now = Utc::now();
        let mut updates = Vec::new();
        let mut params = Vec::new();
        let mut param_count = 1;

        if let Some(name) = &request.name {
            updates.push(format!("name = @P{}", param_count));
            params.push(name.into());
            param_count += 1;
        }

        if let Some(email) = &request.email {
            updates.push(format!("email = @P{}", param_count));
            params.push(email.into());
            param_count += 1;
        }

        if let Some(role) = &request.role {
            updates.push(format!("role = @P{}", param_count));
            params.push(role.into());
            param_count += 1;
        }

        updates.push(format!("updated_at = @P{}", param_count));
        params.push(now.into());
        param_count += 1;

        if updates.is_empty() {
            return self.find_by_id(user_id).await;
        }

        let sql = format!(
            "UPDATE users SET {} WHERE id = @P{}",
            updates.join(", "),
            param_count
        );

        params.push(user_id.into());

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            params,
        );

        self.db.execute(stmt).await?;

        // Buscar usuário atualizado
        self.find_by_id(user_id).await
    }

    pub async fn delete(&self, user_id: &str) -> Result<bool, sea_orm::DbErr> {
        let sql = "DELETE FROM users WHERE id = @P1";

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![user_id.into()],
        );

        let result = self.db.execute(stmt).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn authenticate(&self, request: &LoginRequest) -> Result<Option<User>, sea_orm::DbErr> {
        let user = self.find_by_email(&request.email).await?;
        
        if let Some(user) = user {
            // Verificar senha
            let is_valid = verify(request.password.as_bytes(), &user.password_hash)
                .map_err(|e| sea_orm::DbErr::Custom(format!("Erro ao verificar senha: {}", e)))?;
            
            if is_valid {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn find_by_role(&self, role: &str) -> Result<Vec<User>, sea_orm::DbErr> {
        let sql = r#"
            SELECT id, email, name, role, password_hash, created_at, updated_at
            FROM users
            WHERE role = @P1
            ORDER BY created_at DESC
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![role.into()],
        );

        let results = self.db.query_all(stmt).await?;
        
        let mut users = Vec::new();
        for row in results {
            let id: String = row.try_get("", "id").unwrap_or_default();
            let email: String = row.try_get("", "email").unwrap_or_default();
            let name: String = row.try_get("", "name").unwrap_or_default();
            let role: String = row.try_get("", "role").unwrap_or_default();
            let password_hash: String = row.try_get("", "password_hash").unwrap_or_default();
            let created_at: DateTime<Utc> = row.try_get("", "created_at").unwrap_or_else(|_| Utc::now());
            let updated_at: DateTime<Utc> = row.try_get("", "updated_at").unwrap_or_else(|_| Utc::now());

            users.push(User {
                id,
                email,
                name,
                role,
                password_hash,
                created_at,
                updated_at,
            });
        }

        Ok(users)
    }

    pub async fn count(&self) -> Result<u64, sea_orm::DbErr> {
        let sql = "SELECT COUNT(*) as count FROM users";

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::SqlServer,
            sql,
            vec![],
        );

        let result = self.db.query_one(stmt).await?;
        
        if let Some(row) = result {
            let count: i64 = row.try_get("", "count").unwrap_or(0);
            Ok(count as u64)
        } else {
            Ok(0)
        }
    }
}
