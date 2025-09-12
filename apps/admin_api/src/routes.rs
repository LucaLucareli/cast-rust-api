use crate::modules::video::controllers::upload_video_controller;
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new().route("/video", post(upload_video_controller::handler))
}
