use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UpdateVideoOutputDTO {
    pub id: i32,
}
