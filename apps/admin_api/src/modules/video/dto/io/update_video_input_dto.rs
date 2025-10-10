use crate::modules::video::dto::request::update_video_request_dto::UpdateVideoRequestDTO;
use crate::modules::video::dto::route_params::upload_video_route_params_dto::UploadVideoRouteParamsDTO;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct UpdateVideoInputDTO {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i32>,
    pub release_year: Option<i32>,

    #[serde(default)]
    #[allow(dead_code)]
    pub thumbnail_url: Option<String>,

    #[serde(default)]
    pub is_available: Option<bool>,

    #[serde(default)]
    pub trailer_url: Option<String>,

    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,

    pub id: i32,
}

impl From<(UpdateVideoRequestDTO, UploadVideoRouteParamsDTO)> for UpdateVideoInputDTO {
    fn from((body, params): (UpdateVideoRequestDTO, UploadVideoRouteParamsDTO)) -> Self {
        Self {
            title: body.title,
            description: body.description,
            duration_seconds: body.duration_seconds,
            release_year: body.release_year,
            thumbnail_url: body.thumbnail_url,
            is_available: body.is_available,
            trailer_url: body.trailer_url,
            episode_number: body.episode_number,
            season_number: body.season_number,
            id: params.id,
        }
    }
}
