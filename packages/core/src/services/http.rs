use crate::prelude::*;
use reqwest::Client as ReqwestClient;
use reqwest::Response;
use reqwest::header::CONTENT_TYPE;

const HEAD_EXTENSION: &str = "head";

/// A client for making HTTP requests and caching responses
#[derive(Clone, Debug)]
pub struct HttpClient {
    dir: PathBuf,
}

impl HttpClient {
    #[must_use]
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub async fn get_html(&self, url: &Url) -> Result<Html, Report<HttpError>> {
        let path = self.get(url, Some(HTML_EXTENSION)).await?;
        let contents = read_to_string(&path)
            .await
            .change_context(HttpError::ReadCache)
            .attach_path(path)?;
        Ok(Html::parse_document(&contents))
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &Url) -> Result<T, Report<HttpError>> {
        let path = self.get(url, Some(JSON_EXTENSION)).await?;
        let file = File::open(&path)
            .change_context(HttpError::OpenCache)
            .attach_path(&path)?;
        let reader = BufReader::new(file);
        let result = serde_json::from_reader(reader)
            .change_context(HttpError::Deserialize)
            .attach_path(path);
        if result.is_err() {
            self.remove(url, Some(JSON_EXTENSION)).await;
        }
        result
    }

    pub async fn head(&self, url: &Url) -> Result<String, Report<HttpError>> {
        let path = self.get_cache_path(url, Some(HEAD_EXTENSION));
        if path.exists() {
            trace!("HEAD cache HIT: {url}");
            read_to_string(&path)
                .await
                .change_context(HttpError::ReadCache)
                .attach_path(path)
        } else {
            trace!("HEAD cache MISS: {url}");
            self.head_to_cache(url, &path).await
        }
    }

    pub async fn get(
        &self,
        url: &Url,
        extension: Option<&str>,
    ) -> Result<PathBuf, Report<HttpError>> {
        let path = self.get_cache_path(url, extension);
        if path.exists() {
            trace!("Cache HIT: {url}");
        } else {
            trace!("Cache MISS: {url}");
            self.download_to_cache(url, &path).await?;
        }
        Ok(path)
    }

    pub async fn remove(&self, url: &Url, extension: Option<&str>) -> bool {
        let path = self.get_cache_path(url, extension);
        let exists = path.exists();
        if exists {
            trace!("Removing: {}", path.display());
            if let Err(e) = remove_file(&path).await {
                trace!(path = %path.display(), %e, "Failed to remove cache file");
                return false;
            }
        }
        exists
    }

    fn get_cache_path(&self, url: &Url, extension: Option<&str>) -> PathBuf {
        let domain = url.domain().unwrap_or("__unknown");
        let mut segments: PathBuf = url
            .path_segments()
            .expect("url should have path segments")
            .collect();
        if segments == PathBuf::new() {
            segments = PathBuf::from("__root");
        }
        let mut path = self.dir.join(domain).join(segments);
        if let Some(query) = url.query() {
            let mut file_name = path
                .file_name()
                .expect("path should have a filename")
                .to_owned();
            file_name.push(OsString::from("-"));
            file_name.push(OsString::from(encode(query).as_ref()));
            path.set_file_name(file_name);
        }
        if let Some(extension) = extension {
            path.set_extension(extension);
        }
        path
    }

    #[allow(clippy::unused_self)]
    async fn head_to_cache(&self, url: &Url, path: &PathBuf) -> Result<String, Report<HttpError>> {
        create_dir(path).await?;
        let client = ReqwestClient::new();
        trace!("HEAD {url} to {}", path.display());
        let response = client
            .head(url.as_str())
            .send()
            .await
            .change_context(HttpError::Request)
            .attach_url(url)?;
        let content_type = get_content_type(response).unwrap_or_default();
        let mut file = AsyncFile::create(path)
            .await
            .change_context(HttpError::CreateCache)
            .attach_path(path)?;
        file.write_all(content_type.as_bytes())
            .await
            .change_context(HttpError::WriteCache)
            .attach_path(path)?;
        Ok(content_type)
    }

    #[allow(clippy::unused_self)]
    async fn download_to_cache(&self, url: &Url, path: &PathBuf) -> Result<(), Report<HttpError>> {
        create_dir(path).await?;
        let client = ReqwestClient::new();
        trace!("Downloading {url} to {}", path.display());
        let mut response = client
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
        let mut file = AsyncFile::create(path)
            .await
            .change_context(HttpError::CreateCache)
            .attach_path(path)?;
        while let Some(chunk) = response
            .chunk()
            .await
            .change_context(HttpError::Chunk)
            .attach_url(url)?
        {
            file.write_all(&chunk)
                .await
                .change_context(HttpError::WriteCache)
                .attach_path(path)?;
        }
        Ok(())
    }
}

async fn create_dir(path: &Path) -> Result<(), Report<HttpError>> {
    let dir = path
        .parent()
        .expect("cache path should have a parent directory");
    if !dir.exists() {
        trace!("Creating cache directory: {}", dir.display());
        create_dir_all(dir)
            .await
            .change_context(HttpError::CreateDirectory)
            .attach_path(dir)?;
    }
    Ok(())
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

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            dir: PathProvider::default().get_http_dir(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::Value;

    #[tokio::test]
    #[traced_test]
    pub async fn head() {
        // Arrange
        let http = HttpClient::default();
        let url = Url::parse("https://example.com/?abc=123&def=456").expect("url should be valid");
        http.remove(&url, Some(HEAD_EXTENSION)).await;

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "text/html");
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "uses simplecast.com"]
    pub async fn head_xml() {
        // Arrange
        let http = HttpClient::default();
        let url = example_rss_url();
        http.remove(&url, Some(HEAD_EXTENSION)).await;

        // Act
        let result = http.head(&url).await;

        // Assert
        let content_type = result.assert_ok_debug();
        assert_eq!(content_type, "application/xml");
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get() {
        // Arrange
        let http = HttpClient::default();
        let url = Url::parse("https://example.com/?abc=123&def=456").expect("url should be valid");
        let expected = http.get_cache_path(&url, Some(HTML_EXTENSION));
        http.remove(&url, Some(HTML_EXTENSION)).await;

        // Act
        let result = http.get(&url, Some(HTML_EXTENSION)).await;

        // Assert
        let path = result.assert_ok_debug();
        assert_eq!(path, expected);
        assert!(path.exists());
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_html() {
        // Arrange
        let http = HttpClient::default();
        let url = Url::parse("https://example.com").expect("url should be valid");
        http.remove(&url, Some(HTML_EXTENSION)).await;

        // Act
        let result = http.get_html(&url).await;

        // Assert
        let _html = result.assert_ok_debug();
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "uses ipinfo.io"]
    pub async fn get_json() {
        // Arrange
        let http = HttpClient::default();
        let url = Url::parse("https://ipinfo.io").expect("url should be valid");
        http.remove(&url, Some(JSON_EXTENSION)).await;

        // Act
        let result = http.get_json::<Value>(&url).await;

        // Assert
        let _json = result.assert_ok_debug();
    }
}
