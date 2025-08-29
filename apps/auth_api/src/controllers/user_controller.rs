// controllers/user_controller.rs
use crate::dto::user_dto::CreateUserDto;
use crate::services::user_service;
use axum::Json;

pub async fn get_users() -> Json<Vec<String>> {
    let users = user_service::fetch_users().await;
    Json(users)
}

pub async fn create_user(Json(payload): Json<CreateUserDto>) -> Json<String> {
    let id = user_service::create_user(payload).await;
    Json(id)
}
