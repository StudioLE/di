use crate::prelude::*;
use error_stack::bail;

pub struct MountProvider;

impl Service for MountProvider {
    type Error = ServiceError;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        bail!(ServiceError::NoService)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_service_none() {
        // Arrange
        let services = ServiceProvider::new();
        let _logger = init_test_logger();

        // Act
        let result = services.get_service::<MountProvider>().await;

        // Assert
        assert!(result.is_err());
    }
}
