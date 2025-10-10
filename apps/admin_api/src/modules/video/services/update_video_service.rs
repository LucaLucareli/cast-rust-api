use std::sync::Arc;

use shared::modules::{
    app_state::AppState, database::repositories::videos_repository::UpdateVideoRequest,
    validation::validate_release_year::validate_release_year,
};

use crate::modules::video::dto::io::{
    update_video_input_dto::UpdateVideoInputDTO, update_video_output_dto::UpdateVideoOutputDTO,
};

pub enum UpdateVideoError {
    Validation(String),
    Database(String),
    NotFound(String),
}

pub async fn execute(
    input: UpdateVideoInputDTO,
    state: Arc<AppState>,
) -> Result<UpdateVideoOutputDTO, UpdateVideoError> {
    if let Err(err) = validate_release_year(&input.release_year) {
        return Err(UpdateVideoError::Validation(format!(
            "Ano de lançamento inválido: {}",
            err.message.unwrap_or_default()
        )));
    };

    let video = state
        .video_repo
        .find_by_id(input.id)
        .await
        .map_err(|e| UpdateVideoError::Database(format!("Erro ao buscar vídeo: {}", e)))?;

    if video.is_none() {
        return Err(UpdateVideoError::NotFound(format!(
            "Vídeo com id {} não encontrada",
            input.id
        )));
    }

    let video_id = state
        .video_repo
        .update(
            input.id,
            UpdateVideoRequest {
                title: input.title,
                duration_seconds: input.duration_seconds,
                episode_number: input.episode_number,
                season_number: input.season_number,
                is_available: input.is_available,
                release_year: input.release_year,
                description: input.description,
                trailer_url: input.trailer_url,
                series_id: None,
                video_url: None,
                rating: None,
            },
        )
        .await
        .map_err(|e| UpdateVideoError::Database(format!("Erro ao atualizar {}", e)))?;

    Ok(UpdateVideoOutputDTO { id: video_id })
}
