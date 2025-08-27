// controllers/user_controller.rs
use axum::Json;
use crate::services::user_service;
use crate::dto::user_dto::CreateUserDto;

pub async fn get_users() -> Json<Vec<String>> {
    let users = user_service::fetch_users().await;
    Json(users)
}

pub async fn create_user(Json(payload): Json<CreateUserDto>) -> Json<String> {
    let id = user_service::create_user(payload).await;
    Json(id)
}
