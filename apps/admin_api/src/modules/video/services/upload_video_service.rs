use crate::app_state::AppState;
use axum::{extract::Multipart, http::StatusCode, Json};
use futures::StreamExt;
use futures::TryStreamExt;
use serde_json::json;
use shared::modules::validation::validation_layer::ValidationErrorResponse;
use std::sync::Arc;
use tokio_util::io::StreamReader;

pub async fn execute(
    mut multipart: Multipart,
    state: Arc<AppState>,
) -> Result<(), (StatusCode, Json<ValidationErrorResponse>)> {
    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ValidationErrorResponse {
                    message: "Erro ao processar campo multipart".to_string(),
                    errors: json!([format!("{}", e)]),
                }),
            ));
        }
    } {
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

        let result = storage
            .save_video_file(reader, &file_name, &content_type, max_file_size)
            .await;

        match result {
            Ok((blob_url, size)) => {
                println!("Arquivo salvo: {}, tamanho {}", blob_url, size);
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ValidationErrorResponse {
                        message: "Erro ao processar upload".to_string(),
                        errors: json!([format!("Erro ao salvar arquivo: {}", e)]),
                    }),
                ));
            }
        }
    }

    Ok(())
}
