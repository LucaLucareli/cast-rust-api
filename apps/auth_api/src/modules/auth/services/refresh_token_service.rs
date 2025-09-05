use crate::modules::auth::dto::refresh_token_input_dto::RefreshTokenInputDTO;
use crate::modules::auth::dto::refresh_token_output_dto::RefreshTokenOutputDTO;
use shared::modules::app_state::AppState;
use std::sync::Arc;

pub async fn execute(
    state: Arc<AppState>,
    payload: RefreshTokenInputDTO,
) -> Result<RefreshTokenOutputDTO, String> {
    let auth_response = state
        .auth_service
        .refresh_token(&state, payload.refresh_token)
        .await?;

    Ok(RefreshTokenOutputDTO {
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
    })
}
