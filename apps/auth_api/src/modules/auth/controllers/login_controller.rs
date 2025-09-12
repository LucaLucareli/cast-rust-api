use crate::modules::auth::dto::login_input_dto::LoginInputDTO;
use crate::modules::auth::dto::login_output_dto::LoginOutputDTO;
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
    payload: Json<LoginInputDTO>,
) -> Result<Json<ResponseInterface<LoginOutputDTO>>, (StatusCode, Json<ValidationErrorResponse>)> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    match crate::modules::auth::services::login_service::execute(payload, state).await {
        Ok(auth_response) => Ok(Json(ResponseInterface {
            result: Some(auth_response),
            message: None,
        })),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ValidationErrorResponse {
                message: "Falha na autenticação".to_string(),
                errors: serde_json::json!({ "auth": [e] }),
            }),
        )),
    }
}
