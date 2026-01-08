use std::sync::Arc;

use shared::modules::app_state::AppState;

use crate::modules::video::dto::io::{
    find_many_video_input_dto::FindManyVideoInputDTO,
    find_many_video_output_dto::FindManyVideoOutputDTO,
};

pub enum FindManyVideoError {
    Database(String),
}

pub async fn execute(
    FindManyVideoInputDTO {
        name,
        serie_id,
        skip,
        take,
    }: FindManyVideoInputDTO,
    state: Arc<AppState>,
) -> Result<Vec<FindManyVideoOutputDTO>, FindManyVideoError> {
    let videos = state
        .video_repo
        .find_many_videos(name.as_deref(), serie_id, skip, take)
        .await
        .map_err(|e| FindManyVideoError::Database(format!("Erro ao buscar vídeos: {}", e)))?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<FindManyVideoOutputDTO>>();

    Ok(videos)
}
