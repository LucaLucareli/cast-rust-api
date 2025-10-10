use crate::modules::video::{
    dto::{
        io::{
            update_video_input_dto::UpdateVideoInputDTO,
            update_video_output_dto::UpdateVideoOutputDTO,
        },
        request::update_video_request_dto::UpdateVideoRequestDTO,
        route_params::upload_video_route_params_dto::UploadVideoRouteParamsDTO,
    },
    services::update_video_service,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use macros::require_access;
use serde_json::json;
use shared::{
    enums::access_group_enum::AccessGroupEnum,
    modules::{
        app_state::AppState,
        auth::jwt_extractor::AuthenticatedUser,
        response_interface::ResponseInterface,
        validation::validation_layer::{validate_json, ValidatedJson, ValidationErrorResponse},
    },
};
use std::sync::Arc;

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(params): Path<UploadVideoRouteParamsDTO>,
    payload: Json<UpdateVideoRequestDTO>,
) -> Result<
    (StatusCode, Json<ResponseInterface<UpdateVideoOutputDTO>>),
    (StatusCode, Json<ValidationErrorResponse>),
> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    let input: UpdateVideoInputDTO = (payload, params).into();

    match update_video_service::execute(input, state).await {
        Ok(video) => Ok((
            StatusCode::CREATED,
            Json(ResponseInterface {
                result: Some(video),
                message: None,
            }),
        )),
        Err(err) => {
            let (status, msg) = match err {
                update_video_service::UpdateVideoError::Validation(msg) => {
                    (StatusCode::BAD_REQUEST, msg)
                }
                update_video_service::UpdateVideoError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
                update_video_service::UpdateVideoError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, msg)
                }
            };
            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao atualizar".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
