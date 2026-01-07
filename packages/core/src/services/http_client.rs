use crate::prelude::*;
use crate::services::ipinfo::IpInfoProvider;
use crate::services::{HttpCache, HttpRateLimiter};
use reqwest::Client as ReqwestClient;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;

const HEAD_EXTENSION: &str = "head";
const DEFAULT_DOMAIN: &str = "__unknown";

/// A client for making HTTP requests and caching responses.
///
/// `HttpClient` orchestrates HTTP requests with caching and rate limiting:
/// - Uses `HttpCache` for file-based response caching
/// - Uses `HttpRateLimiter` for per-domain rate limiting
/// - Uses `reqwest::Client` for actual HTTP requests
#[derive(Clone)]
pub struct HttpClient {
    cache: Arc<HttpCache>,
    rate_limiter: Arc<HttpRateLimiter>,
    client: ReqwestClient,
}

impl Service for HttpClient {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<ServiceError>> {
        let ipinfo = services.get_service::<IpInfoProvider>().await?;
        ipinfo
            .validate()
            .await
            .change_context(ServiceError::Create)?;
        Ok(Self {
            cache: services.get_service().await?,
            rate_limiter: services.get_service().await?,
            client: ReqwestClient::new(),
        })
    }
}

impl HttpClient {
    /// Fetch HTML content from a URL.
    ///
    /// Returns the parsed HTML document. The response is cached for future requests.
    pub async fn get_html(&self, url: &UrlWrapper) -> Result<Html, Report<HttpError>> {
        let path = self.get(url, Some(HTML_EXTENSION)).await?;
        let contents = read_to_string(&path)
            .await
            .change_context(HttpError::ReadCache)
            .attach_path(path)?;
        Ok(Html::parse_document(&contents))
    }

    /// Fetch JSON content from a URL and deserialize it.
    ///
    /// The response is cached. If deserialization fails, the cache entry is removed.
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: &UrlWrapper,
    ) -> Result<T, Report<HttpError>> {
        let path = self.get(url, Some(JSON_EXTENSION)).await?;
        let file = File::open(&path)
            .change_context(HttpError::OpenCache)
            .attach_path(&path)?;
        let reader = BufReader::new(file);
        let result = serde_json::from_reader(reader)
            .change_context(HttpError::Deserialize)
            .attach_path(path);
        if result.is_err() {
            self.cache.remove(url, Some(JSON_EXTENSION)).await;
        }
        result
    }

    /// Perform a HEAD request to get the Content-Type of a URL.
    ///
    /// The Content-Type is cached for future requests.
    pub async fn head(&self, url: &UrlWrapper) -> Result<String, Report<HttpError>> {
        let extension = Some(HEAD_EXTENSION);
        if self.cache.exists(url, extension) {
            trace!("HEAD cache HIT: {url}");
            return self.cache.read_string(url, extension).await;
        }
        trace!("HEAD cache MISS: {url}");
        let domain = url.domain().unwrap_or(DEFAULT_DOMAIN);
        self.rate_limiter.wait_for_permit(domain).await;
        let response = self
            .client
            .head(url.as_str())
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)?;
        let content_type = get_content_type(response).unwrap_or_default();
        self.cache
            .write_string(url, extension, &content_type)
            .await?;
        Ok(content_type)
    }

    /// Fetch content from a URL and cache it.
    ///
    /// Returns the path to the cached file. If the content is already cached,
    /// returns immediately without making a network request.
    pub async fn get(
        &self,
        url: &UrlWrapper,
        extension: Option<&str>,
    ) -> Result<PathBuf, Report<HttpError>> {
        if self.cache.exists(url, extension) {
            trace!(%url, "Cache HIT");
            return Ok(self.cache.get_path(url, extension));
        }
        trace!(%url, "Cache MISS");
        let domain = url.domain().unwrap_or(DEFAULT_DOMAIN);
        self.rate_limiter.wait_for_permit(domain).await;
        let mut response = self
            .client
            .get(url.as_str())
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)?;
        if !response.status().is_success() {
            let report = Report::new(HttpError::Status(response.status().as_u16()))
                .attach(format!("URL: {url}"));
            return Err(report);
        }
        let path = self
            .cache
            .write_response(url, extension, &mut response)
            .await?;
        Ok(path)
    }

    /// Download a file from a URL to a destination path.
    ///
    /// The file is first cached, then either hard-linked or copied to the destination.
    pub async fn download(
        &self,
        url: &UrlWrapper,
        destination_path: PathBuf,
        hardlink: bool,
    ) -> Result<(), Report<HttpError>> {
        let extension = destination_path.extension().and_then(|e| e.to_str());
        let source_path = self.get(url, extension).await?;
        hardlink_or_copy(source_path, destination_path, hardlink).await?;
        Ok(())
    }
}

fn get_content_type(response: Response) -> Option<String> {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)?
        .to_str()
        .ok()?
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_lowercase();
    Some(content_type)
}

pub async fn hardlink_or_copy(
    source: PathBuf,
    destination: PathBuf,
    hardlink: bool,
) -> Result<(), Report<HttpError>> {
    create_parent_dir_if_not_exist(&destination)
        .await
        .change_context(HttpError::CreateDestinationDirectory)?;
    if destination.exists() {
        remove_file(&destination)
            .await
            .change_context(HttpError::RemoveExisting)?;
    }
    let result = if hardlink {
        trace!(
            source = %source.display(),
            destination = %destination.display(),
            "Hard linking file"
        );
        hard_link(&source, &destination).await
    } else {
        trace!(
            source = %source.display(),
            destination = %destination.display(),
            "Copying file"
        );
        copy(&source, &destination).await.map(|_| ())
    };
    result.change_context(HttpError::Copy).attach_with(|| {
        format!(
            "Source: {}\nDestination: {}",
            source.display(),
            destination.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::time::{Duration, Instant};

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn head() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url =
            UrlWrapper::from_str("https://example.com/?abc=123&def=456").expect("valid test URL");
        http.cache.remove(&url, Some(HEAD_EXTENSION)).await;
        let _logger = init_test_logger();

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "text/html");
    }

    #[tokio::test]
    #[ignore = "uses simplecast.com"]
    pub async fn head_xml() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = example_rss_url();
        http.cache.remove(&url, Some(HEAD_EXTENSION)).await;
        let _logger = init_test_logger();

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "application/xml");
    }

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn get() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url =
            UrlWrapper::from_str("https://example.com/?abc=123&def=456").expect("valid test URL");
        let expected = http.cache.get_path(&url, Some(HTML_EXTENSION));
        http.cache.remove(&url, Some(HTML_EXTENSION)).await;
        let _logger = init_test_logger();

        // Act
        let result = http.get(&url, Some(HTML_EXTENSION)).await;

        // Assert
        let path = result.assert_ok_debug();
        assert_eq!(path, expected);
        assert!(path.exists());
    }

    #[tokio::test]
    #[ignore = "uses example.com"]
    pub async fn get_html() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://example.com").expect("valid test URL");
        http.cache.remove(&url, Some(HTML_EXTENSION)).await;
        let _logger = init_test_logger();

        // Act
        let result = http.get_html(&url).await;

        // Assert
        let _html = result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "uses ipinfo.io"]
    pub async fn get_json() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://ipinfo.io/json").expect("valid test URL");
        http.cache.remove(&url, Some(JSON_EXTENSION)).await;
        let _logger = init_test_logger();

        // Act
        let result = http.get_json::<Value>(&url).await;

        // Assert
        let _json = result.assert_ok_debug();
    }

    #[tokio::test]
    #[ignore = "requires network to prime cache"]
    async fn cache_hits_not_rate_limited() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let url = UrlWrapper::from_str("https://example.com").expect("valid test URL");
        let _logger = init_test_logger();
        let result = http.get(&url, Some(HTML_EXTENSION)).await;
        assert!(result.is_ok(), "Failed to prime cache");

        // Act
        let start = Instant::now();
        for _ in 0..100 {
            let result = http.get(&url, Some(HTML_EXTENSION)).await;
            assert!(result.is_ok(), "Cache hit failed");
        }
        let elapsed = start.elapsed();

        // Assert
        assert!(
            elapsed < Duration::from_millis(500),
            "Expected cache hits to be instant, elapsed: {elapsed:?}"
        );
    }

    #[tokio::test]
    #[ignore = "requires network"]
    async fn domains_isolated() {
        // Arrange
        let services = ServiceProvider::new();
        let http = services
            .get_service::<HttpClient>()
            .await
            .expect("should be able to get HttpClient");
        let _logger = init_test_logger();
        for i in 0..10 {
            let url1 = UrlWrapper::from_str(&format!("https://example.com/isolated-{i}"))
                .expect("valid test URL");
            let url2 = UrlWrapper::from_str(&format!("https://httpbin.org/isolated-{i}"))
                .expect("valid test URL");
            http.cache.remove(&url1, Some(HTML_EXTENSION)).await;
            http.cache.remove(&url2, Some(HTML_EXTENSION)).await;
        }
        let http1 = http.clone();
        let http2 = http.clone();

        // Act
        let start = Instant::now();
        let task1 = tokio::spawn(async move {
            for i in 0..6 {
                let url = UrlWrapper::from_str(&format!("https://example.com/isolated-{i}"))
                    .expect("valid test URL");
                let _ = http1.get(&url, Some(HTML_EXTENSION)).await;
            }
        });
        let task2 = tokio::spawn(async move {
            for i in 0..6 {
                let url = UrlWrapper::from_str(&format!("https://httpbin.org/isolated-{i}"))
                    .expect("valid test URL");
                let _ = http2.get(&url, Some(HTML_EXTENSION)).await;
            }
        });
        let _ = tokio::try_join!(task1, task2);
        let elapsed = start.elapsed();

        // Assert
        assert!(
            elapsed < Duration::from_secs(3),
            "Expected parallel execution with independent rate limits, elapsed: {elapsed:?}"
        );
    }
}
