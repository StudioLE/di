use super::migration::Migrator;
use crate::prelude::*;
use sea_orm::*;
use sea_orm_migration::MigratorTrait;

pub struct MetadataRepository {
    pub(super) db: DatabaseConnection,
}

impl MetadataRepository {
    pub async fn new(path: PathBuf) -> Result<Self, Report<ServiceError>> {
        let connect_options = ConnectOptions::new(format!("sqlite://{}?mode=rwc", path.display()));
        let db = Database::connect(connect_options)
            .await
            .change_context(ServiceError::DatabaseConnection)?;
        Ok(Self { db })
    }

    pub async fn migrate(&self) -> Result<(), Report<ServiceError>> {
        Migrator::up(&self.db, None)
            .await
            .change_context(ServiceError::DatabaseMigration)
    }
}
