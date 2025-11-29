use super::migration::Migrator;
use crate::prelude::*;
use sea_orm::*;
use sea_orm_migration::MigratorTrait;

pub struct MetadataRepository {
    pub(crate) db: DatabaseConnection,
}

impl Service for MetadataRepository {
    type Error = MetadataRepositoryCreateError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        let paths: Arc<PathProvider> = services
            .get_service()
            .await
            .expect("PathProvider should be available");
        let metadata = MetadataRepository::new(paths.get_metadata_db_path()).await?;
        metadata.migrate().await?;
        Ok(metadata)
    }
}

#[derive(Clone, Debug, Error)]
pub enum MetadataRepositoryCreateError {
    #[error("Unable to connect to database")]
    DatabaseConnection,
    #[error("Unable to migrate database")]
    DatabaseMigration,
}

impl MetadataRepository {
    pub async fn new(path: PathBuf) -> Result<Self, Report<MetadataRepositoryCreateError>> {
        let connect_options = ConnectOptions::new(format!("sqlite://{}?mode=rwc", path.display()));
        let db = Database::connect(connect_options)
            .await
            .change_context(MetadataRepositoryCreateError::DatabaseConnection)?;
        Ok(Self { db })
    }

    pub async fn migrate(&self) -> Result<(), Report<MetadataRepositoryCreateError>> {
        Migrator::up(&self.db, None)
            .await
            .change_context(MetadataRepositoryCreateError::DatabaseMigration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlformat::{FormatOptions, QueryParams, format};

    #[tokio::test]
    pub async fn migrate() {
        // Arrange
        let path = TempDirectory::default()
            .create()
            .expect("Should be able to create a temp dir")
            .join(METADATA_DB);
        let metadata = MetadataRepository::new(path)
            .await
            .expect("Should be able to create a repository");

        // Act
        metadata.migrate().await.assert_ok_debug();

        // Assert
        assert_snapshot!(get_db_structure(&metadata).await);
    }

    async fn get_db_structure(metadata: &MetadataRepository) -> String {
        let statement = Statement::from_string(
            DbBackend::Sqlite,
            "SELECT sql FROM sqlite_master WHERE sql IS NOT NULL",
        );
        let creates: Vec<String> = metadata
            .db
            .query_all_raw(statement)
            .await
            .expect("sqlite_master query should not fail")
            .iter()
            .map(|result| {
                let sql = result
                    .try_get::<String>("", "sql")
                    .expect("should be able to get sql");
                format(&sql, &QueryParams::None, &FormatOptions::default())
            })
            .collect();
        creates.join("\n\n")
    }
}
