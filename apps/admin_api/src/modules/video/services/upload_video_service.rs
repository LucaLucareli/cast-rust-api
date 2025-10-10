use crate::app_state::AppState;
use crate::modules::video::dto::route_params::upload_video_route_params_dto::UploadVideoRouteParamsDTO;
use axum::extract::Multipart;
use futures::StreamExt;
use futures::TryStreamExt;
use shared::modules::database::repositories::videos_repository::UpdateVideoRequest;
use std::sync::Arc;
use tokio_util::io::StreamReader;

pub enum UploadVideoError {
    Validation(String),
    Database(String),
    NotFound(String),
}

pub async fn execute(
    mut multipart: Multipart,
    params: UploadVideoRouteParamsDTO,
    state: Arc<AppState>,
) -> Result<(), UploadVideoError> {
    let video = state
        .video_repo
        .find_by_id(params.id)
        .await
        .map_err(|e| UploadVideoError::Database(format!("Erro ao buscar vídeo: {}", e)))?
        .ok_or_else(|| {
            UploadVideoError::NotFound(format!("Vídeo com id {} não encontrado", params.id))
        })?;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        UploadVideoError::Validation(format!("Erro ao processar campo multipart: {}", e))
    })? {
        let file_name = field.file_name().unwrap_or("upload.bin").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let stream = field
            .into_stream()
            .map(|res| res.map_err(std::io::Error::other));
        let reader = StreamReader::new(stream);

        let max_file_size = 50 * 1024 * 1024;
        let storage = state.video_storage_service.clone();

        let (blob_url, _size) = storage
            .save_video_file(reader, &file_name, &content_type, max_file_size)
            .await
            .map_err(|e| UploadVideoError::Database(format!("Erro ao salvar arquivo: {}", e)))?;

        if let Some(old_url) = &video.video_url {
            if let Ok(old_blob_name) = storage.get_blob_name_from_url(old_url) {
                if let Err(e) = storage.delete_video(&old_blob_name).await {
                    eprintln!("Falha ao deletar vídeo antigo '{}': {:?}", old_blob_name, e);
                }
            }
        }

        state
            .video_repo
            .update(
                video.id,
                UpdateVideoRequest {
                    video_url: Some(blob_url),
                    title: None,
                    description: None,
                    duration_seconds: None,
                    release_year: None,
                    rating: None,
                    trailer_url: None,
                    is_available: None,
                    series_id: None,
                    episode_number: None,
                    season_number: None,
                },
            )
            .await
            .map_err(|e| {
                UploadVideoError::Database(format!("Erro ao atualizar vídeo no banco: {}", e))
            })?;
    }

    Ok(())
}
