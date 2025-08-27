use axum::Router;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use tokio::net::TcpListener;
use socket2::{Socket, Domain, Type, Protocol};
use tracing_subscriber;
use tokio::signal;

mod controllers;
mod dto;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("AUTH_API_PORT")
        .unwrap_or_else(|_| "2402".to_string())
        .parse()
        .expect("PORT deve ser um número");

    let ip: std::net::IpAddr = host.parse().expect("IP inválido");
    let addr = SocketAddr::new(ip, port);

    let app = Router::new()
        .nest("/auth", routes::create_router())
        .route("/", axum::routing::get(|| async { "Auth API - Running" }));

    tracing::info!("Auth API iniciando em http://{}", addr);
    tracing::info!("Endpoints disponíveis:");
    tracing::info!("   - POST /auth/users");
    tracing::info!("   - GET  /auth/users");

    // Criar socket com SO_REUSEADDR
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))
        .expect("Falha ao criar socket");
    socket.set_reuse_address(true).expect("Falha ao setar reuse_address");
    socket.bind(&addr.into()).expect("Falha ao bindar a porta");
    socket.listen(1024).expect("Falha ao colocar em listen");

    let std_listener: StdTcpListener = socket.into();
    std_listener
        .set_nonblocking(true)
        .expect("Falha ao setar non-blocking");

    let tokio_listener = TcpListener::from_std(std_listener).expect("Falha ao criar listener Tokio");

    // Cria uma task que escuta o Ctrl+C e fecha o processo
    let graceful = async {
        signal::ctrl_c().await.expect("Falha ao capturar Ctrl+C");
        tracing::info!("Ctrl+C detectado! Encerrando servidor...");
    };

    // Serve com Axum e aguarda Ctrl+C
    tokio::select! {
        _ = axum::serve(tokio_listener, app) => {},
        _ = graceful => {},
    }

    tracing::info!("Servidor encerrado, porta liberada.");
}
