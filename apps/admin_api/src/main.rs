use axum::{extract::Extension, routing::get, serve, Router};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::TcpListener as StdTcpListener;
use tokio::signal;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use shared::modules::app_state;
use shared::modules::app_state::AppState;
use shared::modules::config::Config;
use shared::modules::interceptors::transform_middleware::transform_middleware;

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
        .layer(axum::middleware::from_fn(transform_middleware))
        .layer(Extension(app_state.clone()));

    let addr = config.admin_api_addr();
    tracing::info!("Admin API iniciando em http://{}", addr);
    tracing::info!("Endpoints disponíveis:");
    tracing::info!("   - POST  /admin/serie");
    tracing::info!("   - POST  /admin/video");
    tracing::info!("   - POST  /admin/video/upload/:id");

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
