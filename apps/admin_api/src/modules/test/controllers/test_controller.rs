use crate::app_state::AppState;
use axum::{extract::Extension, http::StatusCode, Json};
use macros::require_access;
use serde::Serialize;
use shared::enums::access_group_enum::AccessGroupEnum;
use shared::modules::auth::jwt_extractor::AuthenticatedUser;
use shared::modules::response_interface::ResponseInterface;
use shared::modules::validation::validation_layer::ValidationErrorResponse;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(_state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Json<ResponseInterface<String>>, (StatusCode, Json<ValidationErrorResponse>)> {
    Ok(Json(ResponseInterface {
        result: Some("ok".to_string()),
        message: Some("Rota protegida com sucesso!".to_string()),
    }))
}
