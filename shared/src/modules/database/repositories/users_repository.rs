use crate::enums::access_group_enum::AccessGroupEnum;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, FromQueryResult,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::database::schema::users;
use crate::modules::database::schema::users_access_groups;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub access_group_ids: Vec<i32>,
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct AuthUserWithGroups {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub access_group_id: i32,
}

impl UsersRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, request: CreateUserRequest) -> Result<users::Model, DbErr> {
        let now = Utc::now().naive_utc();
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|e| DbErr::Custom(format!("Erro ao hash da senha: {}", e)))?;

        let user = users::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            email: Set(request.email),
            name: Set(request.name),
            role: Set(request.role.unwrap_or_else(|| "Viewer".to_string())),
            password_hash: Set(password_hash),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
            profile_picture_url: Set(None),
            subscription_status: Set(None),
            subscription_expires_at: Set(None),
        };

        user.insert(&self.db).await
    }

    pub async fn find_by_id(&self, user_id: &str) -> Result<Option<users::Model>, DbErr> {
        users::Entity::find_by_id(user_id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<AuthUser>, DbErr> {
        let rows: Vec<AuthUserWithGroups> = users::Entity::find()
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Name)
            .column(users::Column::Email)
            .column(users::Column::PasswordHash)
            .column(users_access_groups::Column::AccessGroupId)
            .left_join(users_access_groups::Entity)
            .filter(users::Column::Email.eq(email))
            .into_model::<AuthUserWithGroups>()
            .all(&self.db)
            .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let access_group_ids: Vec<i32> = rows.iter().map(|r| r.access_group_id).collect();

        let first = &rows[0];

        Ok(Some(AuthUser {
            id: first.id.clone(),
            name: first.name.clone(),
            email: first.email.clone(),
            password_hash: first.password_hash.clone(),
            access_group_ids,
        }))
    }

    pub async fn find_all(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<users::Model>, DbErr> {
        let mut query = users::Entity::find().order_by_desc(users::Column::CreatedAt);

        if let Some(offset) = offset {
            query = query.offset(offset);
        }
        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query.all(&self.db).await
    }

    pub async fn find_access_groups(&self, user_id: &str) -> Result<Vec<AccessGroupEnum>, DbErr> {
        let access_groups: Vec<i32> = users_access_groups::Entity::find()
            .select_only()
            .column(users_access_groups::Column::AccessGroupId)
            .filter(users_access_groups::Column::UserId.eq(user_id))
            .into_values::<i32, users_access_groups::Column>()
            .all(&self.db)
            .await?;

        Ok(access_groups
            .into_iter()
            .map(AccessGroupEnum::from) // converte i32 -> AccessGroupEnum
            .collect())
    }

    pub async fn update(
        &self,
        user_id: &str,
        request: UpdateUserRequest,
    ) -> Result<Option<users::Model>, DbErr> {
        if let Some(user) = self.find_by_id(user_id).await? {
            let mut active_model: users::ActiveModel = user.into();
            if let Some(name) = request.name {
                active_model.name = Set(name);
            }
            if let Some(email) = request.email {
                active_model.email = Set(email);
            }
            if let Some(role) = request.role {
                active_model.role = Set(role);
            }
            active_model.updated_at = Set(Some(Utc::now().naive_utc()));

            let updated = active_model.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, user_id: &str) -> Result<bool, DbErr> {
        if let Some(user) = self.find_by_id(user_id).await? {
            let active_model: users::ActiveModel = user.into();
            let res = active_model.delete(&self.db).await?;
            Ok(res.rows_affected > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn authenticate(&self, request: &LoginRequest) -> Result<Option<AuthUser>, DbErr> {
        if let Some(auth_user) = self.find_by_email(&request.email).await? {
            if verify(&request.password, &auth_user.password_hash)
                .map_err(|e| DbErr::Custom(format!("Erro ao verificar senha: {}", e)))?
            {
                return Ok(Some(auth_user));
            }
        }
        Ok(None)
    }

    pub async fn find_by_role(&self, role: &str) -> Result<Vec<users::Model>, DbErr> {
        users::Entity::find()
            .filter(users::Column::Role.eq(role.to_string()))
            .order_by_desc(users::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        users::Entity::find().count(&self.db).await
    }
}
