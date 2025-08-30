use crate::modules::database::schema::videos;
use crate::modules::database::schema::videos::Model as VideoModel;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::{DatabaseConnection, DbErr, Set};
use serde::Deserialize;
use uuid::Uuid;

// ------------------------------
// Request structs
// ------------------------------
#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: String,
    pub duration_seconds: i32,
    pub release_year: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub duration_seconds: Option<i32>,
    pub release_year: Option<i32>,
    pub rating: Option<f64>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub trailer_url: Option<String>,
    pub is_featured: Option<bool>,
    pub is_available: Option<bool>,
}

// ------------------------------
// Repository
// ------------------------------
pub struct VideosRepository {
    db: DatabaseConnection,
}

impl VideosRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // Create video
    pub async fn create(&self, request: CreateVideoRequest) -> Result<VideoModel, DbErr> {
        let now = Utc::now().naive_utc();
        let video = videos::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            title: Set(request.title),
            description: Set(Some(request.description)),
            duration_seconds: Set(request.duration_seconds),
            release_year: Set(request.release_year),
            rating: Set(0.0),
            thumbnail_url: Set(request.thumbnail_url),
            video_url: Set(request.video_url),
            trailer_url: Set(request.trailer_url),
            is_featured: Set(false),
            is_available: Set(true),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        };

        video.insert(&self.db).await
    }

    // Find by ID
    pub async fn find_by_id(&self, video_id: &str) -> Result<Option<VideoModel>, DbErr> {
        videos::Entity::find_by_id(video_id.to_string())
            .one(&self.db)
            .await
    }

    // Find all (with pagination)
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

    // Find featured
    pub async fn find_featured(&self, limit: Option<u64>) -> Result<Vec<VideoModel>, DbErr> {
        let mut query = videos::Entity::find()
            .filter(videos::Column::IsAvailable.eq(true))
            .filter(videos::Column::IsFeatured.eq(true))
            .order_by_desc(videos::Column::Rating)
            .order_by_desc(videos::Column::CreatedAt);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        query.all(&self.db).await
    }

    // Find by title
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

    // Update video
    pub async fn update(
        &self,
        video_id: &str,
        request: UpdateVideoRequest,
    ) -> Result<Option<VideoModel>, DbErr> {
        if let Some(video) = videos::Entity::find_by_id(video_id.to_string())
            .one(&self.db)
            .await?
        {
            let mut active_model: videos::ActiveModel = video.into();

            if let Some(title) = request.title {
                active_model.title = Set(title);
            }
            if let Some(description) = request.description {
                active_model.description = Set(Some(description));
            }
            if let Some(duration) = request.duration_seconds {
                active_model.duration_seconds = Set(duration);
            }
            if let Some(release_year) = request.release_year {
                active_model.release_year = Set(Some(release_year));
            }
            if let Some(rating) = request.rating {
                active_model.rating = Set(rating);
            }
            if let Some(thumbnail_url) = request.thumbnail_url {
                active_model.thumbnail_url = Set(Some(thumbnail_url));
            }
            if let Some(video_url) = request.video_url {
                active_model.video_url = Set(Some(video_url));
            }
            if let Some(trailer_url) = request.trailer_url {
                active_model.trailer_url = Set(Some(trailer_url));
            }
            if let Some(is_featured) = request.is_featured {
                active_model.is_featured = Set(is_featured);
            }
            if let Some(is_available) = request.is_available {
                active_model.is_available = Set(is_available);
            }

            active_model.updated_at = Set(Some(Utc::now().naive_utc()));

            let updated = active_model.update(&self.db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    // Delete video
    pub async fn delete(&self, video_id: &str) -> Result<bool, DbErr> {
        if let Some(video) = videos::Entity::find_by_id(video_id.to_string())
            .one(&self.db)
            .await?
        {
            let active_model: videos::ActiveModel = video.into();
            active_model
                .delete(&self.db)
                .await
                .map(|res| res.rows_affected > 0)
        } else {
            Ok(false)
        }
    }

    // Count available videos
    pub async fn count(&self) -> Result<u64, DbErr> {
        let count = videos::Entity::find()
            .filter(videos::Column::IsAvailable.eq(true))
            .count(&self.db)
            .await?;
        Ok(count)
    }
}
