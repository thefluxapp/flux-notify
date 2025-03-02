pub use sea_orm_migration::prelude::*;

mod m20250218_160814_create_web_pushes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250218_160814_create_web_pushes::Migration)]
    }
}
