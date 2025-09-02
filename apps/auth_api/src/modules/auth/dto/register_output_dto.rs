use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RegisterOutputDTO {
    pub access_token: String,
    pub refresh_token: String,
}
