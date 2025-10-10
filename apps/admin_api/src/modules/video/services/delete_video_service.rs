use std::sync::Arc;

use shared::modules::app_state::AppState;

use crate::modules::video::dto::io::delete_video_input_dto::DeleteVideoInputDTO;

pub enum DeleteVideoError {
    Database(String),
    NotFound(String),
    Storage(String),
}

pub async fn execute(
    input: DeleteVideoInputDTO,
    state: Arc<AppState>,
) -> Result<(), DeleteVideoError> {
    let video = state
        .video_repo
        .find_by_id(input.id)
        .await
        .map_err(|e| DeleteVideoError::Database(format!("Erro ao buscar vídeo: {}", e)))?
        .ok_or_else(|| {
            DeleteVideoError::NotFound(format!("Vídeo com id {} não encontrado", input.id))
        })?;

    state
        .video_repo
        .delete(input.id)
        .await
        .map_err(|e| DeleteVideoError::Database(format!("Erro ao deletar vídeo: {}", e)))?;

    if let Some(old_url) = &video.video_url {
        if let Ok(old_blob_name) = state.video_storage_service.get_blob_name_from_url(old_url) {
            if let Err(e) = state
                .video_storage_service
                .delete_video(&old_blob_name)
                .await
            {
                DeleteVideoError::Storage(format!(
                    "Falha ao deletar vídeo {} antigo '{}': {:?}",
                    video.title, old_blob_name, e
                ));
            }
        }
    }

    Ok(())
}
