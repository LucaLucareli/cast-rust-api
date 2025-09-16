use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
pub struct CreateVideoInputDTO {
    #[validate(length(min = 3, message = "O título deve ter pelo menos 3 caracteres"))]
    pub title: String,

    #[validate(length(max = 500, message = "A descrição não pode ter mais de 500 caracteres"))]
    pub description: Option<String>,

    #[validate(range(min = 60, message = "Deve ter no mínimo 1 minuto (60 segundos)"))]
    pub duration_seconds: i32,

    pub release_year: Option<i32>,

    #[validate(url(message = "A thumbnail deve ser uma URL válida"))]
    pub thumbnail_url: Option<String>,

    #[serde(default)]
    pub is_available: bool,

    #[validate(url(message = "O trailer deve ser uma URL válida"))]
    pub trailer_url: Option<String>,

    #[validate(range(min = 1, message = "O ID da série deve ser positivo"))]
    pub series_id: Option<i32>,

    #[validate(range(min = 1, message = "O número do episódio deve ser positivo"))]
    pub episode_number: Option<i32>,

    #[validate(range(min = 1, message = "O número da temporada deve ser positivo"))]
    pub season_number: Option<i32>,
}
