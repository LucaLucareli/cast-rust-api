use crate::modules::video::dto::route_params::delete_video_route_params_dto::DeleteVideoRouteParamsDTO;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct DeleteVideoInputDTO {
    pub id: i32,
}

impl From<DeleteVideoRouteParamsDTO> for DeleteVideoInputDTO {
    fn from(params: DeleteVideoRouteParamsDTO) -> Self {
        Self { id: params.id }
    }
}
