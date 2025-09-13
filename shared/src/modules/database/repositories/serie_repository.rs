use crate::modules::database::schema::series;
use crate::modules::database::schema::series::Model as SerieModel;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateSerieRequest {
    pub title: String,
    pub description: Option<String>,
    pub release_year: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub is_featured: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

pub struct SerieRepository {
    db: DatabaseConnection,
}

impl SerieRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, request: CreateSerieRequest) -> Result<SerieModel, DbErr> {
        let now = Utc::now().naive_utc();
        let video = series::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            title: Set(request.title),
            description: Set(request.description),
            release_year: Set(request.release_year),
            thumbnail_url: Set(request.thumbnail_url),
            is_featured: Set(request.is_featured),
            created_at: Set(now),
            updated_at: Set(now),
        };

        video.insert(&self.db).await
    }
}
