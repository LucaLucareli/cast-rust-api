use axum::http::Result;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub log_level: String,
    pub auth_api_port: u16,
    pub admin_api_port: u16,
    pub viewer_api_port: u16,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_expiry_hours: u64,
    pub jwt_refresh_expiry_days: u64,

    pub azure_cast_rustaccount_name: String,
    pub azure_cast_rustaccount_key: String,
    pub azure_cast_rustblob_port: u16,
    pub azure_cast_rustqueue_port: u16,
    pub azure_cast_rusttable_port: u16,
    pub azure_cast_rust_video_container: String,
    pub azure_cast_rust_storage_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            auth_api_port: std::env::var("AUTH_API_PORT")
                .unwrap_or_else(|_| "2402".to_string())
                .parse()
                .unwrap_or(2402),
            admin_api_port: std::env::var("ADMIN_API_PORT")
                .unwrap_or_else(|_| "1608".to_string())
                .parse()
                .unwrap_or(1608),
            viewer_api_port: std::env::var("VIEWER_API_PORT")
                .unwrap_or_else(|_| "1606".to_string())
                .parse()
                .unwrap_or(1606),
            jwt_access_secret: std::env::var("JWT_ACCESS_SECRET")
                .unwrap_or_else(|_| "your-access-secret-key-here".to_string()),
            jwt_refresh_secret: std::env::var("JWT_REFRESH_SECRET")
                .unwrap_or_else(|_| "your-refresh-secret-key-here".to_string()),
            jwt_access_expiry_hours: std::env::var("JWT_ACCESS_EXPIRY_HOURS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),
            jwt_refresh_expiry_days: std::env::var("JWT_REFRESH_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),

            azure_cast_rustaccount_name: std::env::var("AZURE_CAST_RUST_ACCOUNT_NAME")
                .unwrap_or_else(|_| "devstoreaccount1".to_string()),
            azure_cast_rustaccount_key: std::env::var("AZURE_CAST_RUST_ACCOUNT_KEY")
                .unwrap_or_else(|_| "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==".to_string()),
            azure_cast_rustblob_port: std::env::var("AZURE_CAST_RUST_BLOB_PORT")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()
                .unwrap_or(10000),
            azure_cast_rustqueue_port: std::env::var("AZURE_CAST_RUST_QUEUE_PORT")
                .unwrap_or_else(|_| "10001".to_string())
                .parse()
                .unwrap_or(10001),
            azure_cast_rusttable_port: std::env::var("AZURE_CAST_RUST_TABLE_PORT")
                .unwrap_or_else(|_| "10002".to_string())
                .parse()
                .unwrap_or(10002),
            azure_cast_rust_video_container: std::env::var("AZURE_CAST_RUST_VIDEO_CONTAINER")
                .unwrap_or_else(|_| "video".to_string()),
            azure_cast_rust_storage_url: std::env::var("AZURE_CAST_RUST_STORAGE_URL")
                .unwrap_or_else(|_| "http://0.0.0.0:10000/devstoreaccount1".to_string()),
        })
    }

    pub fn auth_api_addr(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.auth_api_port))
    }

    pub fn admin_api_addr(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.admin_api_port))
    }

    pub fn viewer_api_addr(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.viewer_api_port))
    }
}
