use axum::{extract::Extension, routing::get, serve, Router};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::TcpListener as StdTcpListener;
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use sea_orm::Database;
use shared::modules::app_state;
use shared::modules::auth::AuthService;
use shared::modules::config::Config;
use shared::modules::database::repositories::users_repository::UsersRepository;

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

    // Serviços e repositórios
    let auth_service = Arc::new(AuthService::new(
        config.jwt_access_secret.clone(),
        config.jwt_refresh_secret.clone(),
        config.jwt_access_expiry_hours,
        config.jwt_refresh_expiry_days,
    ));

    let db_conn = Database::connect(&config.database_url).await?;
    let users_repo = Arc::new(UsersRepository::new(db_conn));

    let app_state = Arc::new(app_state::AppState::new(auth_service, users_repo));

    // Router
    let app = Router::new()
        .nest("/auth", routes::create_router())
        .route("/", get(|| async { "Auth API - Running" }))
        .layer(Extension(app_state.clone()));

    let addr = config.auth_api_addr();
    tracing::info!("Auth API iniciando em http://{}", addr);
    tracing::info!("Endpoints disponíveis:");
    tracing::info!("   - POST /auth/users");
    tracing::info!("   - GET  /auth/users");
    tracing::info!("   - POST  /auth/users/login");

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
