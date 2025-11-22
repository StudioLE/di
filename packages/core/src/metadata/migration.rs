use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;

mod m20251119_001_create_podcasts_table;
mod m20251119_002_create_episodes_table;

pub(super) struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251119_001_create_podcasts_table::Migration),
            Box::new(m20251119_002_create_episodes_table::Migration),
        ]
    }
}
