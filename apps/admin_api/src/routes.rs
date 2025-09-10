use crate::modules::test::controllers::test_controller;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/test", get(test_controller::handler))
}
