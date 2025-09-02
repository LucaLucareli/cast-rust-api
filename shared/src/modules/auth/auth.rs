use crate::modules::app_state::AppState;
use crate::{
    enums::access_group_enum::AccessGroupEnum,
    modules::database::repositories::users_repository::{LoginRequest, UsersRepository},
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Estrutura do payload do token (semelhante ao TokenInfoDto)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub access_groups: Vec<AccessGroupEnum>,
}

/// Estrutura do token JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub name: String,
    pub access_groups: Vec<AccessGroupEnum>,
}

/// Representa o usuário interno
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub access_groups: Vec<AccessGroupEnum>,
}

/// Resposta do login
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

/// Serviço de autenticação
pub struct AuthService {
    access_secret: String,
    refresh_secret: String,
    access_expiry_hours: u64,
    refresh_expiry_days: u64,
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl AuthService {
    pub fn new(
        access_secret: String,
        refresh_secret: String,
        access_expiry_hours: u64,
        refresh_expiry_days: u64,
    ) -> Self {
        Self {
            access_secret,
            refresh_secret,
            access_expiry_hours,
            refresh_expiry_days,
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registrar usuário
    pub async fn register(
        &self,
        email: String,
        name: String,
        password: String,
        access_groups: Vec<AccessGroupEnum>,
    ) -> Result<TokenInfo, String> {
        let mut users = self.users.write().await;

        if users.values().any(|u| u.email == email) {
            return Err("Usuário já existe".to_string());
        }

        let user_id = Uuid::new_v4().to_string();
        let password_hash =
            hash(password.as_bytes(), DEFAULT_COST).map_err(|_| "Erro ao criptografar senha")?;

        let user = User {
            id: user_id.clone(),
            email: email.clone(),
            name: name.clone(),
            password_hash,
            access_groups: access_groups.clone(),
        };

        users.insert(user_id.clone(), user);

        Ok(TokenInfo {
            id: user_id,
            email,
            name,
            access_groups,
        })
    }

    /// Login e geração de tokens
    pub async fn login(
        &self,
        state: &AppState,
        email: String,
        password: String,
    ) -> Result<AuthResponse, String> {
        let login_request = LoginRequest {
            email: email.clone(),
            password,
        };

        let user_model = UsersRepository::authenticate(&state.user_repo, &login_request)
            .await
            .map_err(|e| format!("Erro ao acessar o banco de dados: {}", e))?
            .ok_or("Usuário ou senha inválidos")?;

        let access_groups: Vec<AccessGroupEnum> = user_model
            .access_group_ids
            .into_iter()
            .map(|id| (id as i32).into()) // se id for i32, From<i32> já funciona
            .collect();

        println!("{:?}", access_groups);

        let user = User {
            id: user_model.id.clone(),
            email: user_model.email.clone(),
            name: user_model.name.clone(),
            password_hash: user_model.password_hash.clone(),
            access_groups,
        };

        // Gera tokens
        let access_token = self.generate_access_token(&user)?;
        let refresh_token = self.generate_refresh_token(&user)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
        })
    }

    /// Validar access token
    pub fn validate_access_token(&self, token: &str) -> Result<TokenInfo, String> {
        let key = DecodingKey::from_secret(self.access_secret.as_ref());
        let token_data = decode::<Claims>(token, &key, &Validation::default())
            .map_err(|_| "Token inválido".to_string())?;

        Ok(TokenInfo {
            id: token_data.claims.sub,
            email: token_data.claims.email,
            name: token_data.claims.name,
            access_groups: token_data.claims.access_groups,
        })
    }

    /// Validar refresh token e gerar novo access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String, String> {
        let key = DecodingKey::from_secret(self.refresh_secret.as_ref());
        let token_data = decode::<Claims>(refresh_token, &key, &Validation::default())
            .map_err(|_| "Refresh token inválido".to_string())?;

        let user_id = token_data.claims.sub;
        let users = self.users.read().await;
        let user = users.get(&user_id).ok_or("Usuário não encontrado")?;

        self.generate_access_token(user)
    }

    fn generate_access_token(&self, user: &User) -> Result<String, String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.access_expiry_hours as i64);

        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            access_groups: user.access_groups.clone(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let key = EncodingKey::from_secret(self.access_secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|_| "Erro ao gerar access token".to_string())
    }

    fn generate_refresh_token(&self, user: &User) -> Result<String, String> {
        let now = Utc::now();
        let exp = now + Duration::days(self.refresh_expiry_days as i64);

        let claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            access_groups: user.access_groups.clone(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let key = EncodingKey::from_secret(self.refresh_secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|_| "Erro ao gerar refresh token".to_string())
    }
}
