use crate::prelude::*;

#[derive(Default)]
pub struct IpInfoProvider {
    options: AppOptions,
    http: HttpClient,
}

impl IpInfoProvider {
    #[must_use]
    pub fn new(options: AppOptions, http: HttpClient) -> Self {
        Self { options, http }
    }

    pub async fn validate(&self) -> Result<(), Report<ServiceError>> {
        if self.options.expect_ip.is_none() && self.options.expect_country.is_none() {
            return Ok(());
        }
        let info = self.get().await?;
        let validation = info.validate(&self.options);
        if validation.is_empty() {
            return Ok(());
        }
        let report = validation
            .into_iter()
            .fold(Report::new(ServiceError::ValidateIp), |report, error| {
                report.attach(error)
            });
        Err(report)
    }

    async fn get(&self) -> Result<IpInfo, Report<ServiceError>> {
        let ip_url = Url::parse("https://ipinfo.io").expect("URL should be valid");
        self.http.remove(&ip_url, Some(JSON_EXTENSION)).await;
        self.http
            .get_json(&ip_url)
            .await
            .change_context(ServiceError::IpRequest)
    }
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
        // Act
        let result = ServiceProvider::create().await;

        // Assert
        let _services = result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    async fn validate_none() {
        // Arrange
        let mut ipinfo = IpInfoProvider::default();
        ipinfo.options.expect_ip = None;
        ipinfo.options.expect_country = None;

        // Act
        let result = ipinfo.validate().await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    async fn validate_invalid() {
        // Arrange
        let mut ipinfo = IpInfoProvider::default();
        ipinfo.options.expect_ip = Some("203.0.113.1".to_owned());
        ipinfo.options.expect_country = Some("INVALID".to_owned());

        // Act
        let result = ipinfo.validate().await;

        // Assert
        let _report = result.assert_err_debug();
    }
}
