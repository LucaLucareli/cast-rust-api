use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct FindManyVideoOutputDTO {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub duration_seconds: i32,
    pub is_available: bool,
    pub rating: f64,
    pub series_id: Option<i32>,
    pub episode_number: Option<i32>,
    pub season_number: Option<i32>,
    pub release_year: Option<i32>,
}
