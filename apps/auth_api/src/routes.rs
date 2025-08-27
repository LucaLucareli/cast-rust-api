use axum::routing::{get, post};
use axum::Router;

use crate::controllers::{user_controller};

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(user_controller::get_users))
        .route("/users", post(user_controller::create_user))
}
