use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateVideoOutputDTO {
    pub id: i32,
}
