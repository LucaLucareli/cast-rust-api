use crate::modules::auth::controllers::{login_controller, register_controller};
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/users/login", post(login_controller::handler))
        .route("/users/register", post(register_controller::handler))
}
