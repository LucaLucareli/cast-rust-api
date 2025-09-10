use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};

use crate::modules::auth::{user_from_jwt, User};
use crate::modules::config::Config;
use crate::modules::validation::validation_layer::ValidationErrorResponse;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub User);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ValidationErrorResponse>);

    #[allow(clippy::needless_lifetimes)]
    fn from_request_parts<'a>(
        parts: &'a mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>> + Send
    {
        Box::pin(async move {
            use axum::http::header;

            let config = Config::from_env().map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ValidationErrorResponse {
                        message: "Configuração inválida".to_string(),
                        errors: serde_json::json!([
                            "Não foi possível ler as variáveis de ambiente"
                        ]),
                    }),
                )
            })?;

            let token = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or_else(|| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(ValidationErrorResponse {
                            message: "Token ausente".to_string(),
                            errors: serde_json::json!(["Authorization header ausente"]),
                        }),
                    )
                })?;

            let user = user_from_jwt(token.trim(), &config.jwt_access_secret).map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ValidationErrorResponse {
                        message: "Token inválido".to_string(),
                        errors: serde_json::json!(["JWT inválido ou expirado"]),
                    }),
                )
            })?;

            Ok(AuthenticatedUser(user))
        })
    }
}
