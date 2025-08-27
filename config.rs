use std::net::SocketAddr;
use serde::Deserialize;
use crate::errors::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub api_port: u16,
    pub log_level: String,
    pub auth_api_port: u16,
    pub admin_api_port: u16,
    pub viewer_api_port: u16,
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_expiry_hours: u64,
    pub jwt_refresh_expiry_days: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlserver://localhost:1433/rust_cast_db".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            api_port: std::env::var("API_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),
            log_level: std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            auth_api_port: std::env::var("AUTH_API_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            admin_api_port: std::env::var("ADMIN_API_PORT")
                .unwrap_or_else(|_| "3002".to_string())
                .parse()
                .unwrap_or(3002),
            viewer_api_port: std::env::var("VIEWER_API_PORT")
                .unwrap_or_else(|_| "3003".to_string())
                .parse()
                .unwrap_or(3003),
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

    pub fn main_api_addr(&self) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], self.api_port))
    }
}
