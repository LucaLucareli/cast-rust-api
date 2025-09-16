use crate::modules::video::dto::request::create_video_request_dto::CreateVideoRequestDTO;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct CreateVideoInputDTO {
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: i32,
    pub release_year: Option<i32>,

    #[serde(default)]
    #[allow(dead_code)]
    pub thumbnail_url: Option<String>,

    #[serde(default)]
    pub is_available: bool,

    #[serde(default)]
    pub trailer_url: Option<String>,
    pub series_id: Option<i32>,
    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,
}

impl From<CreateVideoRequestDTO> for CreateVideoInputDTO {
    fn from(dto: CreateVideoRequestDTO) -> Self {
        Self {
            title: dto.title,
            description: dto.description,
            duration_seconds: dto.duration_seconds,
            release_year: dto.release_year,
            thumbnail_url: dto.thumbnail_url,
            is_available: dto.is_available,
            trailer_url: dto.trailer_url,
            series_id: dto.series_id,
            episode_number: dto.episode_number,
            season_number: dto.season_number,
        }
    }
}
