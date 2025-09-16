use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UploadVideoRouteParamsDTO {
    #[validate(range(min = 1, message = "O ID deve ser positivo"))]
    pub id: i32,
}
