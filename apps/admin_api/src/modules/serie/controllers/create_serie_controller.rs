use crate::app_state::AppState;
use crate::modules::serie::dto::create_serie_input_dto::CreateSerieInputDTO;
use crate::modules::serie::dto::create_serie_output_dto::CreateSerieOutputDTO;
use crate::modules::serie::services::create_serie_service;
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
    payload: Json<CreateSerieInputDTO>,
) -> Result<
    Json<ResponseInterface<CreateSerieOutputDTO>>,
    (StatusCode, Json<ValidationErrorResponse>),
> {
    let ValidatedJson(payload) = validate_json(payload).await?;

    match create_serie_service::execute(payload, state).await {
        Ok(id) => Ok(Json(ResponseInterface {
            result: Some(id),
            message: Some("Série criada com sucesso!".to_string()),
        })),
        Err(err) => {
            let (status, msg) = match err {
                create_serie_service::CreateSerieError::Validation(msg) => {
                    (StatusCode::BAD_REQUEST, msg)
                }
                create_serie_service::CreateSerieError::Database(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
            };
            Err((
                status,
                Json(ValidationErrorResponse {
                    message: "Erro ao criar série".to_string(),
                    errors: json!([msg]),
                }),
            ))
        }
    }
}
