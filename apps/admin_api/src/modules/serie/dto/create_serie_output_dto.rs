use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateSerieOutputDTO {
    pub id: i32,
}
