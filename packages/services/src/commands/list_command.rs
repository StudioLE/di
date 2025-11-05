use crate::prelude::*;

pub struct ListCommand {
    paths: PathProvider,
    metadata: MetadataStore,
}

impl ListCommand {
    #[must_use]
    pub fn new(paths: PathProvider, metadata: MetadataStore) -> Self {
        Self { paths, metadata }
    }

    pub async fn execute(&self) -> Result<Vec<Podcast>, Report<ListError>> {
        let path = self.paths.get_metadata_dir();
        let mut dir = read_dir(&path)
            .await
            .change_context(ListError::ReadDirectory)
            .attach_path(&path)?;
        let mut podcasts = Vec::new();
        while let Some(entry) = dir
            .next_entry()
            .await
            .change_context(ListError::ReadEntry)?
        {
            if entry.path().extension() != Some(OsStr::new("yml")) {
                continue;
            }
            let id = entry
                .path()
                .file_stem()
                .expect("should have a file stem")
                .to_string_lossy()
                .to_string();
            let podcast = self
                .metadata
                .get(&id)
                .change_context(ListError::GetPodcast)
                .attach_with(|| format!("Podcast ID: {id}"))?;
            podcasts.push(podcast);
        }
        Ok(podcasts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn list_command() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let command = ListCommand::new(services.paths, services.metadata);

        // Act
        let result = command.execute().await;

        // Assert
        result.assert_ok_debug();
    }
}
