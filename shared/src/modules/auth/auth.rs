use crate::modules::app_state::AppState;
use crate::modules::database::repositories::users_repository::CreateUserRequest;
use crate::{
    enums::access_group_enum::AccessGroupEnum,
    modules::database::repositories::users_repository::{LoginRequest, UsersRepository},
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct RefreshClaims {
    sub: String,
    iat: i64,
    exp: i64,
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
        }
    }

    /// Registrar usuário
    pub async fn register(
        &self,
        state: &AppState,
        email: String,
        name: String,
        password: String,
        access_groups: Vec<AccessGroupEnum>,
    ) -> Result<AuthResponse, String> {
        if UsersRepository::find_by_email(&state.user_repo, &email)
            .await
            .map_err(|e| format!("Erro ao acessar o banco: {}", e))?
            .is_some()
        {
            return Err("Falha ao cadastrar o usuário".to_string());
        }

        let password_hash =
            hash(password.as_bytes(), DEFAULT_COST).map_err(|_| "Erro ao criptografar senha")?;

        let new_user = CreateUserRequest {
            email: email.clone(),
            name: name.clone(),
            password_hash: password_hash.clone(),
            access_group_ids: access_groups.iter().copied().map(|g| g as i32).collect(),
        };

        let user_model = UsersRepository::create(&state.user_repo, new_user)
            .await
            .map_err(|e| format!("Erro ao criar usuário: {}", e))?;

        let user = User {
            id: user_model.id.clone(),
            email: user_model.email.clone(),
            name: user_model.name.clone(),
            password_hash: user_model.password_hash.clone(),
            access_groups,
        };

        let access_token = self.generate_access_token(&user)?;
        let refresh_token = self.generate_refresh_token(&user)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
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

        #[allow(clippy::unnecessary_cast)]
        let access_groups: Vec<AccessGroupEnum> = user_model
            .access_group_ids
            .into_iter()
            .map(|id| (id as i32).into()) // se id for i32, From<i32> já funciona
            .collect();

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
    pub async fn refresh_token(
        &self,
        state: &AppState,
        refresh_token: String,
    ) -> Result<AuthResponse, String> {
        let key = DecodingKey::from_secret(self.refresh_secret.as_ref());

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data =
            decode::<RefreshClaims>(&refresh_token, &key, &validation).map_err(|err| match *err
                .kind()
            {
                jsonwebtoken::errors::ErrorKind::InvalidToken => "Token inválido".to_string(),
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expirado".to_string(),
                _ => "Erro ao validar refresh token".to_string(),
            })?;

        let user_id = token_data.claims.sub;
        let user_model = UsersRepository::find_by_id(&state.user_repo, &user_id)
            .await
            .map_err(|e| format!("Erro ao acessar o banco de dados: {}", e))?
            .ok_or("Usuário não encontrado")?;

        let user = User {
            id: user_model.id,
            email: user_model.email,
            name: user_model.name,
            password_hash: user_model.password_hash,
            access_groups: user_model.access_groups,
        };

        let access_token = self.generate_access_token(&user)?;
        let refresh_token = self.generate_refresh_token(&user)?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
        })
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

        let claims = RefreshClaims {
            sub: user.id.clone(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
        };

        let key = EncodingKey::from_secret(self.refresh_secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|_| "Erro ao gerar refresh token".to_string())
    }
}
