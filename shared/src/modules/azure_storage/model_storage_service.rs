use anyhow::{anyhow, Result};
use azure_core::auth::Secret;
use azure_core::base64;
use azure_core::Url;
use azure_storage::prelude::SasToken;
use azure_storage::shared_access_signature::service_sas::{
    BlobSasPermissions, BlobSharedAccessSignature, BlobSignedResource, SasKey,
};
use azure_storage_blobs::blob::operations::PutBlockBlobResponse;
use azure_storage_blobs::blob::BlobBlockType;
use azure_storage_blobs::blob::BlockList;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::ops::Range;
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
        let blob_service_client = ClientBuilder::emulator().blob_service_client();

        let container_client =
            Arc::new(blob_service_client.container_client(config.container_name.clone()));

        match container_client
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
        {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("{:?}", e);
                if !msg.contains("ContainerAlreadyExists") {
                    return Err(anyhow::anyhow!(
                        "Não foi possível criar o container: {:?}",
                        e
                    ));
                }
            }
        }

        container_client
            .get_properties()
            .into_future()
            .await
            .map_err(|e| anyhow::anyhow!("Não foi possível acessar o container: {:?}", e))?;

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

    pub async fn upload_stream_with_blocks<R: AsyncRead + Unpin + Send>(
        &self,
        mut stream: R,
        blob_name: &str,
        buffer_size: usize,
        blob_content_type: Option<&str>,
        max_file_size: Option<u64>,
    ) -> Result<(String, u64)> {
        let mut total_bytes: u64 = 0;
        let mut block_ids: Vec<String> = Vec::new();

        let content_type = blob_content_type
            .map(|s| s.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let blob_client = self.container_client.blob_client(blob_name);

        let mut buffer = vec![0u8; buffer_size];
        let mut block_number = 0;

        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            total_bytes += n as u64;

            if let Some(max) = max_file_size {
                if total_bytes > max {
                    return Err(anyhow::anyhow!(
                        "The file exceeds the size of {} bytes",
                        max
                    ));
                }
            }

            let chunk = buffer[..n].to_vec();

            let block_id = format!("{:08}", block_number);
            let block_id_base64 = base64::encode(&block_id);

            blob_client
                .put_block(block_id_base64.clone(), chunk)
                .into_future()
                .await?;

            block_ids.push(block_id_base64);
            block_number += 1;
        }

        let mut block_list = BlockList { blocks: Vec::new() };
        for id in block_ids {
            block_list.blocks.push(BlobBlockType::Latest(id.into()));
        }

        blob_client
            .put_block_list(block_list)
            .content_type(content_type)
            .into_future()
            .await?;

        Ok((blob_client.url()?.to_string(), total_bytes))
    }

    pub async fn get_blob_range(
        &self,
        blob_name: &str,
        range: Option<Range<u64>>,
    ) -> Result<Vec<u8>> {
        let blob_client = self.get_blob_client(blob_name);

        let mut get_blob = blob_client.get();

        if let Some(r) = range {
            get_blob = get_blob.range(r);
        }

        let mut stream = get_blob.into_stream();

        let mut bytes = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let chunk_bytes = chunk.data.collect().await?;
            bytes.extend_from_slice(&chunk_bytes);
        }

        Ok(bytes)
    }
}
