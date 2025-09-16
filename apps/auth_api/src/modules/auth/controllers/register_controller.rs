use crate::modules::auth::dto::register_input_dto::RegisterInputDTO;
use crate::modules::auth::dto::register_output_dto::RegisterOutputDTO;
use crate::AppState;
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
    payload: Json<RegisterInputDTO>,
) -> Result<
    (StatusCode, Json<ResponseInterface<RegisterOutputDTO>>),
    (StatusCode, Json<ValidationErrorResponse>),
> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    match crate::modules::auth::services::register_service::execute(payload, state).await {
        Ok(auth_response) => Ok((
            StatusCode::CREATED,
            Json(ResponseInterface {
                result: Some(auth_response),
                message: None,
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ValidationErrorResponse {
                message: "Falha ao cadastrar o usuário".to_string(),
                errors: serde_json::json!({ "auth": [e] }),
            }),
        )),
    }
}
