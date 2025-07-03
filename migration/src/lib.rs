pub use sea_orm_migration::prelude::*;

pub mod m20250630_000001_create_therapist_table;
pub mod m20250701_182901_create_client_table;
pub mod m20250701_190743_create_client_therapist_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250630_000001_create_therapist_table::Migration),
            Box::new(m20250701_182901_create_client_table::Migration),
            Box::new(m20250701_190743_create_client_therapist_table::Migration),
        ]
    }
}
