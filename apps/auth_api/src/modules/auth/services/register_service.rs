use crate::modules::auth::dto::register_input_dto::RegisterInputDTO;
use crate::modules::auth::dto::register_output_dto::RegisterOutputDTO;
use shared::enums::access_group_enum::AccessGroupEnum;
use shared::modules::app_state::AppState;
use std::sync::Arc;

pub async fn execute(
    payload: RegisterInputDTO,
    state: Arc<AppState>,
) -> Result<RegisterOutputDTO, String> {
    let auth_response = state
        .auth_service
        .register(
            &state,
            payload.email,
            payload.name,
            payload.password,
            vec![AccessGroupEnum::VIEWER],
        )
        .await?;

    Ok(RegisterOutputDTO {
        access_token: auth_response.access_token,
        refresh_token: auth_response.refresh_token,
    })
}
