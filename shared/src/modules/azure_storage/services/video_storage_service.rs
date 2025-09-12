use anyhow::Result;
use slug::slugify;
use std::ops::Range;
use std::sync::Arc;
use tokio::io::AsyncRead;
use url::Url;
use uuid::Uuid;

use crate::modules::azure_storage::model_storage_service::{StorageService, StorageServiceConfig};
use crate::modules::config::Config;

pub struct VideoStorageService {
    storage_service: Arc<StorageService>,
    storage_url: String,
}

impl VideoStorageService {
    pub async fn new(config: &Config) -> Result<Self> {
        let storage_service = StorageService::new(StorageServiceConfig {
            account_name: config.azure_cast_rustaccount_name.clone(),
            account_key: config.azure_cast_rustaccount_key.clone(),
            container_name: config.azure_cast_rust_video_container.clone(),
        })
        .await?;

        Ok(Self {
            storage_service: Arc::new(storage_service),
            storage_url: config.azure_cast_rust_storage_url.clone(),
        })
    }

    pub fn generate_video_blob_path(&self, file_name: &str) -> String {
        format!("{}-{}", Uuid::new_v4(), slugify(file_name))
    }

    pub async fn get_url_to_upload_video(&self, file_name: &str) -> Result<Url> {
        let blob_name = self.generate_video_blob_path(file_name);
        self.storage_service
            .get_signed_url_for_upload(&blob_name)
            .await
    }

    pub async fn save_video_file<R: AsyncRead + Unpin + Send>(
        &self,
        mut file: R,
        file_name: &str,
        blob_content_type: &str,
        max_file_size: u64,
    ) -> Result<(String, u64)> {
        let blob_name = self.generate_video_blob_path(file_name);

        let (_blob_url, total_bytes) = self
            .storage_service
            .upload_stream(
                &mut file,
                &blob_name,
                8 * 1024, // buffer de 8KB
                Some(blob_content_type),
                Some(max_file_size),
            )
            .await?;

        let full_url = format!("{}/{}", self.storage_url, blob_name);

        Ok((full_url, total_bytes))
    }

    pub fn get_blob_name_from_url(&self, url: &str) -> Result<String> {
        let parsed_url = Url::parse(url)?;

        let path = parsed_url.path();

        let blob_name = path.split('/').next_back().unwrap_or("").to_string();

        Ok(blob_name)
    }

    pub async fn get_blob_video_parsed(&self, blob_name: &str) -> Result<Vec<u8>> {
        self.storage_service.read_model(blob_name).await
    }

    pub async fn stream_video(
        &self,
        blob_name: &str,
        range: Option<Range<u64>>,
    ) -> Result<Vec<u8>> {
        let storage_service = &self.storage_service;

        let data = match range {
            Some(r) => storage_service.get_blob_range(blob_name, Some(r)).await?,
            None => storage_service.get_blob_range(blob_name, None).await?,
        };

        Ok(data)
    }
}
