use crate::modules::auth::controllers::{login_controller, user_controller};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/users",
            get(user_controller::get_users).post(user_controller::create_user),
        )
        .route("/users/login", post(login_controller::login_handler))
}
