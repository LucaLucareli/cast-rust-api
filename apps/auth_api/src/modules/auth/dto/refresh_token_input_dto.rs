use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenInputDTO {
    #[validate(length(min = 1, message = "refreshToken não pode ser vazio"))]
    pub refresh_token: String,
}
