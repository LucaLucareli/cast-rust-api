use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LoginOutputDTO {
    pub access_token: String,
    pub refresh_token: String,
}
