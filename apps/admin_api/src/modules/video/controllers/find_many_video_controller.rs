use std::sync::Arc;

use axum::{extract::Query, http::StatusCode, Extension, Json};
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

use crate::modules::video::{
    dto::{
        io::find_many_video_output_dto::FindManyVideoOutputDTO,
        query_params::find_many_video_query_params_dto::FindManyVideoQueryParamsDTO,
    },
    services::find_many_video_service,
};

#[axum::debug_handler]
#[require_access(AccessGroupEnum::ADMIN, AccessGroupEnum::SUPER_ADMIN)]
pub async fn handler(
    Extension(state): Extension<Arc<AppState>>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(query): Query<FindManyVideoQueryParamsDTO>,
) -> Result<
    (
        StatusCode,
        Json<ResponseInterface<Vec<FindManyVideoOutputDTO>>>,
    ),
    (StatusCode, Json<ValidationErrorResponse>),
> {
    match find_many_video_service::execute(query.into(), state).await {
        Ok(video) => Ok((
            StatusCode::OK,
            Json(ResponseInterface {
                result: Some(video),
                message: None,
            }),
        )),
        Err(err) => {
            let (status, msg) = match err {
                find_many_video_service::FindManyVideoError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
            };
            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao buscar os vídeos".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
