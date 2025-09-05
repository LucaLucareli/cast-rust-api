use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RefreshTokenOutputDTO {
    pub access_token: String,
    pub refresh_token: String,
}
