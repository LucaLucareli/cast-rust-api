use crate::modules::serie::controllers::create_serie_controller;
use crate::modules::video::controllers::upload_video_controller;
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/video", post(upload_video_controller::handler))
        .route("/serie", post(create_serie_controller::handler))
}
