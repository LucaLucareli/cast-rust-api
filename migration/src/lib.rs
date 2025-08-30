pub use sea_orm_migration::prelude::*;

mod m20250828_140352_create_streaming_schema;
mod m20250828_313242_create_index_and_dadas;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250828_140352_create_streaming_schema::Migration),
            Box::new(m20250828_313242_create_index_and_dadas::Migration),
        ]
    }
}
