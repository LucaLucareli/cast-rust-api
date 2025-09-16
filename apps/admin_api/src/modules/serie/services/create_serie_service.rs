use crate::modules::serie::dto::create_serie_input_dto::CreateSerieInputDTO;
use crate::modules::serie::dto::create_serie_output_dto::CreateSerieOutputDTO;
use crate::AppState;
use shared::modules::database::repositories::serie_repository::CreateSerieRequest;
use shared::modules::validation::validate_release_year::validate_release_year;
use std::sync::Arc;

pub enum CreateSerieError {
    Validation(String),
    Database(String),
}

pub async fn execute(
    payload: CreateSerieInputDTO,
    state: Arc<AppState>,
) -> Result<CreateSerieOutputDTO, CreateSerieError> {
    if let Err(err) = validate_release_year(&payload.release_year) {
        return Err(CreateSerieError::Validation(format!(
            "Ano de lançamento inválido: {}",
            err.message.unwrap_or_default()
        )));
    };

    let response = state
        .serie_repo
        .create(CreateSerieRequest {
            title: payload.title,
            description: payload.description,
            release_year: payload.release_year,
            thumbnail_url: payload.thumbnail_url,
            is_featured: payload.is_featured,
        })
        .await
        .map_err(|e| CreateSerieError::Database(format!("Erro ao criar: {}", e)))?;

    Ok(CreateSerieOutputDTO { id: response.id })
}
