use std::sync::Arc;

use anyhow::Result;
use sea_orm::Database;

use crate::modules::auth::AuthService;
use crate::modules::azure_storage::services::video_storage_service::VideoStorageService;
use crate::modules::config::Config;
use crate::modules::database::repositories::{
    serie_repository::SerieRepository, users_repository::UsersRepository,
    videos_repository::VideosRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_repo: Arc<UsersRepository>,
    pub video_repo: Arc<VideosRepository>,
    pub serie_repo: Arc<SerieRepository>,
    pub video_storage_service: Arc<VideoStorageService>,
}

impl AppState {
    pub fn new(
        auth_service: Arc<AuthService>,
        user_repo: Arc<UsersRepository>,
        video_repo: Arc<VideosRepository>,
        serie_repo: Arc<SerieRepository>,
        video_storage_service: Arc<VideoStorageService>,
    ) -> Self {
        Self {
            auth_service,
            user_repo,
            video_repo,
            serie_repo,
            video_storage_service,
        }
    }

    pub async fn init(config: &Config) -> Result<Arc<Self>> {
        let db_conn = Database::connect(&config.database_url).await?;

        let users_repo = Arc::new(UsersRepository::new(db_conn.clone()));
        let video_repo = Arc::new(VideosRepository::new(db_conn.clone()));
        let serie_repo = Arc::new(SerieRepository::new(db_conn));

        let auth_service = Arc::new(AuthService::new(
            config.jwt_access_secret.clone(),
            config.jwt_refresh_secret.clone(),
            config.jwt_access_expiry_hours,
            config.jwt_refresh_expiry_days,
        ));

        let video_storage_service = Arc::new(VideoStorageService::new(config).await?);

        Ok(Arc::new(Self::new(
            auth_service,
            users_repo,
            video_repo,
            serie_repo,
            video_storage_service,
        )))
    }
}
