// // use axum::Router;
// use std::net::SocketAddr;
// // use tokio::net::TcpListener;
// use tracing_subscriber;

// mod controllers;
// mod dto;
// mod routes;
// mod services;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();

    // tracing_subscriber::fmt::init();

    // let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    // let port: u16 = std::env::var("ADMIN_API_PORT")
    //     .unwrap_or("1608".to_string())
    //     .parse()
    //     .expect("PORT deve ser um número");

    // let addr = SocketAddr::from((host.parse::<std::net::IpAddr>().unwrap(), port));

    // // let app = Router::new()
    // //     .nest("/admin", routes::create_router())
    // //     .route("/", axum::routing::get(|| async { "Admin API - Running" }));

    // tracing::info!("Admin API iniciando em http://{}", addr);
    // tracing::info!("Endpoints disponíveis:");

    // let listener = TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
