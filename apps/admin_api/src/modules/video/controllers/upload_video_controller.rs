use crate::app_state::AppState;
use crate::modules::video::services::upload_video_service;
use axum::{extract::Multipart, http::StatusCode, Extension, Json};
use macros::require_access;
use serde_json::json;
use shared::enums::access_group_enum::AccessGroupEnum;
use shared::modules::auth::jwt_extractor::AuthenticatedUser;
use shared::modules::response_interface::ResponseInterface;
use shared::modules::validation::validation_layer::ValidationErrorResponse;
use std::sync::Arc;

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    multipart: Multipart,
) -> Result<Json<ResponseInterface<String>>, (StatusCode, Json<ValidationErrorResponse>)> {
    let result = upload_video_service::execute(multipart, state).await;

    match result {
        Ok(()) => Ok(Json(ResponseInterface {
            result: Some("Upload feito com sucesso!".to_string()),
            message: Some("Upload processado com sucesso!".to_string()),
        })),
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ValidationErrorResponse {
                    message: "Erro ao processar upload".to_string(),
                    errors: json!([format!("Erro ao salvar arquivo: {:?}", e)]),
                }),
            ));
        }
    }
}
