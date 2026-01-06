use crate::prelude::*;

pub struct TestServiceProvider;

impl TestServiceProvider {
    pub async fn create() -> ServiceProvider {
        let temp_dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp dir")
            .join("data");
        let data_dir = temp_dir.join("data");
        create_dir_all(&data_dir)
            .await
            .expect("should be able to create temp data dir");
        let options = AppOptions {
            data_dir: Some(data_dir),
            ..AppOptions::default()
        };
        let metadata = MetadataRepositoryExample::create_in_directory(temp_dir).await;
        let mut services = ServiceProvider::new();
        services.add_instance(options);
        services.add_instance(metadata);
        services
            .with_commands()
            .await
            .expect("should be able to create services with commands")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn service_provider_create() {
        // Arrange
        let services = TestServiceProvider::create().await;

        // Act
        let options = services
            .get_service::<AppOptions>()
            .await
            .expect("should be able to get options");

        // Assert
        let data_dir = options
            .as_ref()
            .clone()
            .data_dir
            .expect("should have data dir");
        assert!(data_dir.exists());
        assert!(data_dir.components().any(|component| {
            let component = component.as_os_str().to_str().unwrap_or_default();
            component.contains("service_provider_create")
        }));
    }
}
