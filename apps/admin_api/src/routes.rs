use crate::modules::serie::controllers::create_serie_controller;
use crate::modules::video::controllers::{create_video_controller, upload_video_controller};
use axum::{routing::post, Router};

pub fn create_router() -> Router {
    Router::new()
        .nest(
            "/video",
            Router::new()
                .route("/", post(create_video_controller::handler))
                .route("/upload", post(upload_video_controller::handler)),
        )
        .nest(
            "/serie",
            Router::new().route("/", post(create_serie_controller::handler)),
        )
}
