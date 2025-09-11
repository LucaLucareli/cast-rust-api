use anyhow::{anyhow, Result};
use azure_core::auth::Secret;
use azure_core::Url;
use azure_storage::prelude::SasToken;
use azure_storage::shared_access_signature::service_sas::{
    BlobSasPermissions, BlobSharedAccessSignature, BlobSignedResource, SasKey,
};
use azure_storage::StorageCredentials;
use azure_storage_blobs::blob::operations::PutBlockBlobResponse;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::io::{AsyncRead, AsyncReadExt};

pub struct StorageServiceConfig {
    pub account_name: String,
    pub account_key: String,
    pub container_name: String,
}

#[allow(dead_code)]
pub struct StorageService {
    container_client: Arc<ContainerClient>,
    account_name: String,
    account_key: String,
    container_name: String,
}

impl StorageService {
    pub async fn new(config: StorageServiceConfig) -> Result<Self> {
        let credentials =
            StorageCredentials::access_key(config.account_name.clone(), config.account_key.clone());

        let blob_service_client =
            BlobServiceClient::new(config.account_name.clone(), credentials.clone());

        let container_client =
            Arc::new(blob_service_client.container_client(config.container_name.clone()));

        container_client
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
            .ok();

        Ok(Self {
            container_client,
            account_name: config.account_name,
            account_key: config.account_key,
            container_name: config.container_name,
        })
    }

    pub fn get_blob_client(&self, blob_name: &str) -> BlobClient {
        self.container_client.blob_client(blob_name)
    }

    pub async fn save_model(&self, blob_name: &str, data: Vec<u8>) -> Result<()> {
        let blob_client = self.get_blob_client(blob_name);
        blob_client.put_block_blob(data).into_future().await?;
        Ok(())
    }

    pub async fn read_model(&self, blob_name: &str) -> Result<Vec<u8>> {
        let blob_client = self.get_blob_client(blob_name);

        let mut stream = blob_client.get().into_stream();

        if let Some(Ok(response)) = stream.next().await {
            let bytes: Vec<u8> = response.data.collect().await?.to_vec();
            Ok(bytes)
        } else {
            Err(anyhow!("Erro ao ler o blob"))
        }
    }

    pub async fn get_signed_url_for_upload(&self, blob_name: &str) -> Result<Url> {
        let expires_on = OffsetDateTime::now_utc() + time::Duration::minutes(10);
        let permissions = BlobSasPermissions {
            write: true,
            ..Default::default()
        };

        let canonicalized_resource = format!("/blob/{}/{}", self.container_name, blob_name);

        let sas_key = SasKey::Key(Secret::new(self.account_key.clone()));

        let sas_token = BlobSharedAccessSignature::new(
            sas_key,
            canonicalized_resource,
            permissions,
            expires_on,
            BlobSignedResource::Blob,
        );

        let blob_url = self
            .get_blob_client(blob_name)
            .url()
            .map_err(|e| anyhow!("Erro ao gerar URL do blob: {:?}", e))?;

        let signed_url_str = format!("{}?{:#?}", blob_url, sas_token.token());

        let signed_url = signed_url_str
            .parse::<Url>()
            .map_err(|e| anyhow!("Erro ao gerar URL assinada: {:?}", e))?;

        Ok(signed_url)
    }

    pub async fn upload_buffer(
        &self,
        data: Vec<u8>,
        blob_name: &str,
        blob_content_type: &str,
    ) -> Result<String> {
        let blob_client = self.container_client.blob_client(blob_name);

        let content_type_owned = blob_content_type.to_owned();

        let _response: PutBlockBlobResponse = blob_client
            .put_block_blob(data)
            .content_type(content_type_owned)
            .into_future()
            .await?;

        Ok(blob_client.url()?.to_string())
    }

    pub async fn upload_stream<R: AsyncRead + Unpin + Send>(
        &self,
        mut stream: R,
        blob_name: &str,
        buffer_size: usize,
        blob_content_type: Option<&str>,
        max_file_size: Option<u64>,
    ) -> Result<(String, u64)> {
        let blob_client = self.container_client.blob_client(blob_name);

        let mut total_bytes: u64 = 0;
        let mut buffer = vec![0u8; buffer_size];

        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            total_bytes += n as u64;

            if let Some(max) = max_file_size {
                if total_bytes > max {
                    return Err(anyhow::anyhow!(
                        "O arquivo excede o tamanho de {} bytes",
                        max
                    ));
                }
            }

            let content_type = blob_content_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "application/octet-stream".to_string());

            blob_client
                .put_block_blob(buffer[..n].to_vec())
                .content_type(content_type)
                .into_future()
                .await?;
        }

        Ok((blob_client.url()?.to_string(), total_bytes))
    }
}
