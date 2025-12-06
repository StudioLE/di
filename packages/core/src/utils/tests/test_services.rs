use crate::prelude::*;

pub struct TestServiceProvider;

impl TestServiceProvider {
    pub async fn create() -> ServiceProvider {
        let mut services = ServiceProvider::new()
            .with_commands()
            .await
            .expect("should be able to create services with commands");
        let temp_dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp dir");
        let options = AppOptions {
            data_dir: Some(temp_dir.join("data")),
            ..AppOptions::default()
        };
        services.add_instance(options);
        let metadata = MetadataRepositoryExample::create_in_directory(temp_dir).await;
        services.add_instance(metadata);
        services
    }
}
