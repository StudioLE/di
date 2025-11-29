use crate::prelude::*;
use reqwest::Client as ReqwestClient;

#[derive(Default)]
pub struct IpInfoProvider {
    options: Arc<AppOptions>,
}

impl Service for IpInfoProvider {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_service().await?))
    }
}

impl IpInfoProvider {
    #[must_use]
    pub fn new(options: Arc<AppOptions>) -> Self {
        Self { options }
    }

    pub async fn validate(&self) -> Result<(), Report<IpInfoError>> {
        if self.options.expect_ip.is_none() && self.options.expect_country.is_none() {
            return Ok(());
        }
        let info = self.get().await.change_context(IpInfoError::IpRequest)?;
        let validation = info.validate(&self.options);
        if validation.is_empty() {
            return Ok(());
        }
        let report = validation
            .into_iter()
            .fold(Report::new(IpInfoError::ValidateIp), |report, error| {
                report.attach(error)
            });
        Err(report)
    }

    async fn get(&self) -> Result<IpInfo, Report<HttpError>> {
        let url = "https://ipinfo.io";
        let client = ReqwestClient::new();
        let response = client
            .get(url)
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_with(|| format!("URL: {url}"))?;
        if !response.status().is_success() {
            let report = Report::new(HttpError::Status(response.status().as_u16()))
                .attach(format!("URL: {url}"));
            return Err(report);
        }
        response.json().await.change_context(HttpError::Deserialize)
    }
}

#[derive(Clone, Debug, Error)]
pub enum IpInfoError {
    #[error("Failed to make request for external IP")]
    IpRequest,
    #[error("IP validation failed")]
    ValidateIp,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct IpInfo {
    ip: String,
    hostname: Option<String>,
    city: String,
    region: String,
    country: String,
    loc: String,
    org: String,
    postal: String,
    timezone: String,
}

impl IpInfo {
    pub fn validate(&self, options: &AppOptions) -> Vec<ValidationError> {
        if options.expect_ip.is_none() && options.expect_country.is_none() {
            return Vec::new();
        }
        let mut errors = Vec::new();
        let values = vec![
            ("IP address", options.expect_ip.clone(), &self.ip),
            (
                "Geolocated country",
                options.expect_country.clone(),
                &self.country,
            ),
        ];
        for (name, expected, actual) in values {
            let Some(expected) = expected else {
                continue;
            };
            let Err(e) = Validate::expect(&expected, actual) else {
                continue;
            };
            errors.push(ValidationError::String(name.to_owned(), e));
        }
        errors
    }
}

impl Display for IpInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} ({}, {}, {})",
            self.ip, self.city, self.region, self.country
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    async fn validate_env() {
        // Arrange
        let services = ServiceProvider::new();
        let ipinfo = services
            .get_service::<IpInfoProvider>()
            .await
            .expect("should be able to get ipinfo");

        // Act
        let result = ipinfo.validate().await;

        // Assert
        result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    async fn validate_none() {
        // Arrange
        let mut services = ServiceProvider::new();
        let options = AppOptions {
            expect_ip: None,
            expect_country: None,
            ..AppOptions::default()
        };
        services.add_instance(options);
        let ipinfo = services
            .get_service::<IpInfoProvider>()
            .await
            .expect("should be able to get ipinfo");

        // Act
        let result = ipinfo.validate().await;

        // Assert
        result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    async fn validate_invalid() {
        // Arrange
        let mut services = ServiceProvider::new();
        let options = AppOptions {
            expect_ip: Some("203.0.113.1".to_owned()),
            expect_country: Some("INVALID".to_owned()),
            ..AppOptions::default()
        };
        services.add_instance(options);
        let ipinfo = services
            .get_service::<IpInfoProvider>()
            .await
            .expect("should be able to get ipinfo");

        // Act
        let result = ipinfo.validate().await;

        // Assert
        let _report = result.assert_err_debug();
    }
}
