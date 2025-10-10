use crate::modules::video::{
    dto::{
        io::delete_video_input_dto::DeleteVideoInputDTO,
        route_params::delete_video_route_params_dto::DeleteVideoRouteParamsDTO,
    },
    services::delete_video_service,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use macros::require_access;
use serde_json::json;
use shared::{
    enums::access_group_enum::AccessGroupEnum,
    modules::{
        app_state::AppState, auth::jwt_extractor::AuthenticatedUser,
        response_interface::ResponseInterface,
        validation::validation_layer::ValidationErrorResponse,
    },
};
use std::sync::Arc;

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Path(params): Path<DeleteVideoRouteParamsDTO>,
) -> Result<(StatusCode, Json<ResponseInterface<()>>), (StatusCode, Json<ValidationErrorResponse>)>
{
    let input: DeleteVideoInputDTO = params.into();

    match delete_video_service::execute(input, state).await {
        Ok(video) => Ok((
            StatusCode::OK,
            Json(ResponseInterface {
                message: Some("Vídeo deletado com sucesso".to_string()),
                result: Some(video),
            }),
        )),
        Err(err) => {
            let (status, msg) = match err {
                delete_video_service::DeleteVideoError::Storage(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
                delete_video_service::DeleteVideoError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
                delete_video_service::DeleteVideoError::NotFound(msg) => {
                    (StatusCode::NOT_FOUND, msg)
                }
            };
            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao deletar".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
