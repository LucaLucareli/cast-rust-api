use serde::Deserialize;

use crate::modules::video::dto::query_params::find_many_video_query_params_dto::FindManyVideoQueryParamsDTO;

#[derive(Debug, Deserialize, Default)]
pub struct FindManyVideoInputDTO {
    pub serie_id: Option<i32>,
    pub name: Option<String>,
    pub skip: Option<i32>,
    pub take: Option<i32>,
}

impl From<FindManyVideoQueryParamsDTO> for FindManyVideoInputDTO {
    fn from(query: FindManyVideoQueryParamsDTO) -> Self {
        Self {
            serie_id: query.serie_id,
            name: query.name,
            skip: query.skip,
            take: query.take,
        }
    }
}
