use crate::app_state::AppState;
use crate::modules::auth::dto::login_input_dto::LoginInputDTO;
use crate::modules::auth::dto::login_output_dto::LoginOutputDTO;
use std::sync::Arc;

pub async fn execute(
    payload: LoginInputDTO,
    state: Arc<AppState>,
) -> Result<LoginOutputDTO, String> {
    let auth_response = state
        .auth_service
        .login(&state, payload.email, payload.password)
        .await?;

    Ok(LoginOutputDTO {
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
    })
}
