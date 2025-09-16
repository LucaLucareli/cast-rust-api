use crate::modules::video::dto::create_video_input_dto::CreateVideoInputDTO;
use crate::modules::video::dto::create_video_output_dto::CreateVideoOutputDTO;
use crate::AppState;
use shared::modules::database::repositories::videos_repository::CreateVideoRequest;
use shared::modules::validation::validate_release_year::validate_release_year;
use std::sync::Arc;

pub enum CreateVideoError {
    Validation(String),
    Database(String),
    NotFound(String),
}

pub async fn execute(
    payload: CreateVideoInputDTO,
    state: Arc<AppState>,
) -> Result<CreateVideoOutputDTO, CreateVideoError> {
    if let Err(err) = validate_release_year(&payload.release_year) {
        return Err(CreateVideoError::Validation(format!(
            "Ano de lançamento inválido: {}",
            err.message.unwrap_or_default()
        )));
    };

    if let Some(series_id) = payload.series_id {
        let serie = state
            .serie_repo
            .find_by_id(series_id)
            .await
            .map_err(|e| CreateVideoError::Database(format!("Erro ao buscar série: {}", e)))?;

        if serie.is_none() {
            return Err(CreateVideoError::NotFound(format!(
                "Série com id {} não encontrada",
                series_id
            )));
        }
    }

    let response = state
        .video_repo
        .create(CreateVideoRequest {
            title: payload.title,
            description: payload.description,
            release_year: payload.release_year,
            duration_seconds: payload.duration_seconds,
            is_available: payload.is_available,
            trailer_url: payload.trailer_url,
            series_id: payload.series_id,
            episode_number: payload.episode_number,
            season_number: payload.season_number,
            video_url: None,
        })
        .await
        .map_err(|e| CreateVideoError::Database(format!("Erro ao criar: {}", e)))?;

    Ok(CreateVideoOutputDTO { id: response.id })
}
