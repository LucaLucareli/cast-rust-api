use crate::app_state::AppState;
use crate::modules::video::dto::io::create_video_input_dto::CreateVideoInputDTO;
use crate::modules::video::dto::io::create_video_output_dto::CreateVideoOutputDTO;
use crate::modules::video::dto::request::create_video_request_dto::CreateVideoRequestDTO;
use crate::modules::video::services::create_video_service;
use axum::{http::StatusCode, Extension, Json};
use macros::require_access;
use serde_json::json;
use shared::enums::access_group_enum::AccessGroupEnum;
use shared::modules::auth::jwt_extractor::AuthenticatedUser;
use shared::modules::response_interface::ResponseInterface;
use shared::modules::validation::validation_layer::{
    validate_json, ValidatedJson, ValidationErrorResponse,
};
use std::sync::Arc;

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    payload: Json<CreateVideoRequestDTO>,
) -> Result<
    (StatusCode, Json<ResponseInterface<CreateVideoOutputDTO>>),
    (StatusCode, Json<ValidationErrorResponse>),
> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    let input: CreateVideoInputDTO = payload.into();

    match create_video_service::execute(input, state).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            Json(ResponseInterface {
                result: Some(id),
                message: Some("Criada com sucesso!".to_string()),
            }),
        )),
        Err(err) => {
            let (status, msg) = match err {
                create_video_service::CreateVideoError::Validation(msg) => {
                    (StatusCode::BAD_REQUEST, msg)
                }
                create_video_service::CreateVideoError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
                create_video_service::CreateVideoError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, msg)
                }
            };
            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao criar".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
