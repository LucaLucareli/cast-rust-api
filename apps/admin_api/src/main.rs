use axum::{extract::Extension, routing::get, serve, Router};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::TcpListener as StdTcpListener;
use tokio::signal;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use axum::http;
use shared::modules::app_state;
use shared::modules::app_state::AppState;
use shared::modules::config::Config;
use shared::modules::interceptors::transform_middleware::transform_middleware;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod modules;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Config
    let config = Config::from_env()?;

    // Logging
    std::env::set_var("RUST_LOG", &config.log_level);
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let app_state = AppState::init(&config).await?;

    // Router
    let app = Router::new()
        .nest("/admin", routes::create_router())
        .route("/", get(|| async { "Admin API - Running" }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                    )
                })
                .on_response(
                    tower_http::trace::DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .layer(axum::middleware::from_fn(transform_middleware))
        .layer(Extension(app_state.clone()));

    let addr = config.admin_api_addr();
    tracing::info!("Admin API iniciando em http://{}", addr);
    tracing::info!("Endpoints disponíveis:");

    // TCP socket
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?;
    let std_listener: StdTcpListener = socket.into();
    std_listener.set_nonblocking(true)?;

    // Shutdown
    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Falha ao capturar Ctrl+C");
        tracing::info!("Ctrl+C detectado! Encerrando servidor...");
    };

    // Serve usando Axum `serve`
    serve(tokio::net::TcpListener::from_std(std_listener)?, app)
        .with_graceful_shutdown(shutdown_signal)
        .await?;

    tracing::info!("Servidor encerrado, porta liberada.");
    Ok(())
}
