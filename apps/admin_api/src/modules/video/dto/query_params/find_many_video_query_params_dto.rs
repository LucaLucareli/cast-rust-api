use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct FindManyVideoQueryParamsDTO {
    #[validate(length(min = 1, message = "O nome não pode ser vazio"))]
    pub name: Option<String>,

    #[validate(range(min = 0, message = "Skip não pode ser negativo"))]
    pub skip: Option<i32>,

    #[validate(range(min = 1, max = 100, message = "Take deve estar entre 1 e 100"))]
    pub take: Option<i32>,

    #[validate(range(min = 1, message = "O ID da serie deve ser positivo"))]
    pub serie_id: Option<i32>,
}
