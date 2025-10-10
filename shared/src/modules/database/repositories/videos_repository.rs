use crate::modules::database::schema::videos;
use crate::modules::database::schema::videos::Model as VideoModel;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: i32,
    pub is_available: bool,
    pub release_year: Option<i32>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub series_id: Option<i32>,
    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i32>,
    pub release_year: Option<i32>,
    pub rating: Option<f64>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub is_available: Option<bool>,
    pub series_id: Option<i32>,
    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,
}

pub struct VideosRepository {
    db: DatabaseConnection,
}

impl VideosRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, request: CreateVideoRequest) -> Result<VideoModel, DbErr> {
        let now = Utc::now().naive_utc();
        let video = videos::ActiveModel {
            id: sea_orm::ActiveValue::NotSet,
            title: Set(request.title),
            description: Set(request.description),
            duration_seconds: Set(request.duration_seconds),
            release_year: Set(request.release_year),
            rating: Set(0.0),
            video_url: Set(request.video_url),
            trailer_url: Set(request.trailer_url),
            is_available: Set(request.is_available),
            created_at: Set(now),
            updated_at: Set(now),
            series_id: Set(request.series_id),
            episode_number: Set(request.episode_number),
            season_number: Set(request.season_number),
        };

        video.insert(&self.db).await
    }

    pub async fn find_by_id(&self, video_id: i32) -> Result<Option<VideoModel>, DbErr> {
        videos::Entity::find_by_id(video_id).one(&self.db).await
    }

    pub async fn find_all(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<VideoModel>, DbErr> {
        let mut query = videos::Entity::find().filter(videos::Column::IsAvailable.eq(true));
        if let Some(offset) = offset {
            query = query.offset(offset);
        }
        if let Some(limit) = limit {
            query = query.limit(limit);
        }
        query
            .order_by_desc(videos::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn find_by_title(
        &self,
        title: &str,
        limit: Option<u64>,
    ) -> Result<Vec<VideoModel>, DbErr> {
        let mut query = videos::Entity::find()
            .filter(videos::Column::IsAvailable.eq(true))
            .filter(videos::Column::Title.contains(title))
            .order_by_desc(videos::Column::Rating)
            .order_by_desc(videos::Column::CreatedAt);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query.all(&self.db).await
    }

    pub async fn update(&self, video_id: i32, request: UpdateVideoRequest) -> Result<i32, DbErr> {
        let mut active_model = videos::ActiveModel {
            id: Set(video_id),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        if let Some(title) = request.title {
            active_model.title = Set(title);
        }
        if request.description.is_some() {
            active_model.description = Set(request.description);
        }
        if let Some(duration) = request.duration_seconds {
            active_model.duration_seconds = Set(duration);
        }
        if request.release_year.is_some() {
            active_model.release_year = Set(request.release_year);
        }
        if let Some(rating) = request.rating {
            active_model.rating = Set(rating);
        }
        if request.video_url.is_some() {
            active_model.video_url = Set(request.video_url);
        }
        if request.trailer_url.is_some() {
            active_model.trailer_url = Set(request.trailer_url);
        }
        if let Some(is_available) = request.is_available {
            active_model.is_available = Set(is_available);
        }
        if request.series_id.is_some() {
            active_model.series_id = Set(request.series_id);
        }
        if request.episode_number.is_some() {
            active_model.episode_number = Set(request.episode_number);
        }
        if request.season_number.is_some() {
            active_model.season_number = Set(request.season_number);
        }

        let updated = active_model.update(&self.db).await?;

        Ok(updated.id)
    }

    pub async fn delete(&self, video_id: i32) -> Result<bool, DbErr> {
        if let Some(video) = videos::Entity::find_by_id(video_id).one(&self.db).await? {
            let active_model: videos::ActiveModel = video.into();
            active_model
                .delete(&self.db)
                .await
                .map(|res| res.rows_affected > 0)
        } else {
            Ok(false)
        }
    }

    pub async fn count(&self) -> Result<u64, DbErr> {
        let count = videos::Entity::find()
            .filter(videos::Column::IsAvailable.eq(true))
            .count(&self.db)
            .await?;
        Ok(count)
    }
}
