use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct CreateSerieInputDTO {
    #[validate(length(
        min = 3,
        message = "O título da série deve ter pelo menos 3 caracteres"
    ))]
    pub title: String,

    #[validate(length(max = 500, message = "A descrição não pode ter mais de 500 caracteres"))]
    pub description: Option<String>,

    pub release_year: Option<i32>,

    #[validate(url(message = "A thumbnail deve ser uma URL válida"))]
    pub thumbnail_url: Option<String>,

    pub is_featured: bool,
}
