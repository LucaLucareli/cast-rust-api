use crate::app_state::AppState;
use crate::modules::auth::dto::refresh_token_input_dto::RefreshTokenInputDTO;
use crate::modules::auth::dto::refresh_token_output_dto::RefreshTokenOutputDTO;
use axum::{extract::Extension, http::StatusCode, Json};
use serde::Serialize;
use shared::modules::response_interface::ResponseInterface;
use shared::modules::validation::validation_layer::{
    validate_json, ValidatedJson, ValidationErrorResponse,
};
use std::sync::Arc;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[axum::debug_handler]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    payload: Json<RefreshTokenInputDTO>,
) -> Result<
    Json<ResponseInterface<RefreshTokenOutputDTO>>,
    (StatusCode, Json<ValidationErrorResponse>),
> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    match crate::modules::auth::services::refresh_token_service::execute(state, payload).await {
        Ok(auth_response) => Ok(Json(ResponseInterface {
            result: Some(auth_response),
            message: None,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ValidationErrorResponse {
                message: "Falha ao recriar o usuário".to_string(),
                errors: serde_json::json!({ "auth": [e] }),
            }),
        )),
    }
}
