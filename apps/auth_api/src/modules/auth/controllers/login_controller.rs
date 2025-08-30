use crate::app_state::AppState;
use crate::modules::auth::dto::login_input_dto::LoginInputDTO;
use crate::modules::auth::dto::login_output_dto::LoginOutputDTO;
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[axum::debug_handler]
pub async fn login_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginInputDTO>,
) -> Result<Json<LoginOutputDTO>, (StatusCode, Json<ErrorResponse>)> {
    match crate::modules::auth::services::login_service::execute(payload, state).await {
        Ok(auth_response) => Ok(Json(auth_response)),
        Err(e) => Err((StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: e }))),
    }
}
