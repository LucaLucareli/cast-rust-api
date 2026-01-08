use serde::Serialize;
use shared::modules::database::repositories::dto::find_many_video_output_dto::FindManyVideoOutputDTO as RepoVideoDTO;

#[derive(Debug, Clone, Serialize)]
pub struct FindManyVideoOutputDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: i32,
    pub is_available: bool,
    pub rating: f64,
    pub series_id: Option<i32>,
    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,
    pub release_year: Option<i32>,
}

impl From<RepoVideoDTO> for FindManyVideoOutputDTO {
    fn from(v: RepoVideoDTO) -> Self {
        Self {
            id: v.id,
            title: v.title,
            description: v.description,
            duration_seconds: v.duration_seconds,
            is_available: v.is_available,
            rating: v.rating,
            series_id: v.series_id,
            episode_number: v.episode_number,
            season_number: v.season_number,
            release_year: v.release_year,
        }
    }
}
