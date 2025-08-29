use sea_orm::{DatabaseConnection, Database};
use crate::config::Config;

pub async fn create_connection(config: &Config) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(&config.database_url).await
}

pub async fn test_connection(connection: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    connection.ping().await
}
