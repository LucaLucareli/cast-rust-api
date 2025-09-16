use crate::app_state::AppState;
use crate::modules::video::dto::route_params::upload_video_route_params_dto::UploadVideoRouteParamsDTO;
use crate::modules::video::services::upload_video_service;
use axum::extract::Path;
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
    Path(params): Path<UploadVideoRouteParamsDTO>,
    multipart: Multipart,
) -> Result<Json<ResponseInterface<String>>, (StatusCode, Json<ValidationErrorResponse>)> {
    match upload_video_service::execute(multipart, params, state).await {
        Ok(()) => Ok(Json(ResponseInterface {
            result: Some("Upload feito com sucesso!".to_string()),
            message: Some("Upload processado com sucesso!".to_string()),
        })),
        Err(err) => {
            let (status, msg) = match err {
                upload_video_service::UploadVideoError::Validation(msg) => {
                    (StatusCode::BAD_REQUEST, msg)
                }
                upload_video_service::UploadVideoError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
                upload_video_service::UploadVideoError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, msg)
                }
            };

            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao processar upload".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
