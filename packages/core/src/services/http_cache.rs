use crate::prelude::*;
use reqwest::Response;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use std::ffi::OsString;

/// Service for managing HTTP response caching.
#[derive(Clone, Debug)]
pub struct HttpCache {
    cache_dir: PathBuf,
}

impl Service for HttpCache {
    type Error = ServiceError;

    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<ServiceError>> {
        let paths: Arc<PathProvider> = services.get_service().await?;
        Ok(Self {
            cache_dir: paths.get_http_dir(),
        })
    }
}

impl HttpCache {
    /// Get the cache file path for a URL with optional file extension.
    ///
    /// The cache path is structured as: `{cache_dir}/{domain}/{url_path}/{query}.{extension}`
    /// - Domain from URL (or `__unknown` if missing)
    /// - URL path segments form directory structure
    /// - Query parameters encoded and appended to filename
    /// - Extension added at end
    #[must_use]
    pub fn get_path(&self, url: &UrlWrapper, extension: Option<&str>) -> PathBuf {
        let domain = url.domain().unwrap_or("__unknown");
        let mut segments: PathBuf = url
            .path_segments()
            .expect("http/https URLs have path segments")
            .collect();
        if segments == PathBuf::new() {
            segments = PathBuf::from("__root");
        }
        let mut path = self.cache_dir.join(domain).join(segments);
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
        assert!(
            is_path_within(&path, &self.cache_dir),
            "URL validation should prevent path escape: {}",
            path.display()
        );
        path
    }

    /// Check if a cached file exists for the given URL.
    #[must_use]
    pub fn exists(&self, url: &UrlWrapper, extension: Option<&str>) -> bool {
        self.get_path(url, extension).exists()
    }

    /// Remove a cached file for the given URL.
    ///
    /// Returns `true` if the file existed and was removed, `false` otherwise.
    pub async fn remove(&self, url: &UrlWrapper, extension: Option<&str>) -> bool {
        let path = self.get_path(url, extension);
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

    /// Read a string from the cache.
    pub async fn read_string(
        &self,
        url: &UrlWrapper,
        extension: Option<&str>,
    ) -> Result<String, Report<HttpError>> {
        let path = self.get_path(url, extension);
        let contents = read_to_string(&path)
            .await
            .change_context(HttpError::ReadCache)
            .attach_path(path)?;
        Ok(contents)
    }

    /// Write a string to the cache.
    pub async fn write_string(
        &self,
        url: &UrlWrapper,
        extension: Option<&str>,
        content: &str,
    ) -> Result<PathBuf, Report<HttpError>> {
        let path = self.get_path(url, extension);
        trace!(%url, path = %path.display(), "Writing string to cache");
        ensure_dir_exists(&path).await?;
        let mut file = AsyncFile::create(&path)
            .await
            .change_context(HttpError::CreateCache)
            .attach_path(&path)?;
        file.write_all(content.as_bytes())
            .await
            .change_context(HttpError::WriteCache)
            .attach_path(&path)?;
        file.sync_all()
            .await
            .change_context(HttpError::SyncCache)
            .attach_path(&path)?;
        Ok(path)
    }

    /// Write a response as chunks to the cache.
    pub async fn write_response(
        &self,
        url: &UrlWrapper,
        extension: Option<&str>,
        response: &mut Response,
    ) -> Result<PathBuf, Report<HttpError>> {
        let path = self.get_path(url, extension);
        trace!(
            %url,
            path = %path.display(),
            content_type = ?response.headers().get(CONTENT_TYPE),
            content_length = ?response.headers().get(CONTENT_LENGTH),
            "Writing response to cache"
        );
        ensure_dir_exists(&path).await?;
        let mut file = AsyncFile::create(&path)
            .await
            .change_context(HttpError::CreateCache)
            .attach_path(&path)?;
        let mut bytes_written = 0;
        while let Some(chunk) = response
            .chunk()
            .await
            .change_context(HttpError::Chunk)
            .attach_url(url)?
        {
            bytes_written += chunk.len();
            file.write_all(&chunk)
                .await
                .change_context(HttpError::WriteCache)
                .attach_path(&path)?;
        }
        file.sync_all()
            .await
            .change_context(HttpError::SyncCache)
            .attach_path(&path)?;
        trace!(
            %url,
            path = %path.display(),
            bytes_written,
            "Finished writing response to cache"
        );
        if bytes_written == 0 {
            let report = Report::new(HttpError::Size).attach(format!("Path: {}", path.display()));
            warn!(%url, path = %path.display(), "Response body was empty. Removing the cache file");
            self.remove(url, extension).await;
            return Err(report);
        }
        Ok(path)
    }
}

/// Ensure the cache directory exists for a given path.
///
/// Creates all parent directories if they don't exist.
async fn ensure_dir_exists(path: &Path) -> Result<(), Report<HttpError>> {
    let dir = path
        .parent()
        .expect("cache path should have a parent directory");
    if !dir.exists() {
        trace!(path = %dir.display(), "Creating cache directory");
        create_dir_all(dir)
            .await
            .change_context(HttpError::CreateCacheDirectory)
            .attach_path(dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cache() -> HttpCache {
        HttpCache {
            cache_dir: PathBuf::from("/test/cache"),
        }
    }

    #[test]
    fn get_path_basic_url() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com/path/to/file").expect("valid test URL");
        let path = cache.get_path(&url, Some("html"));
        assert_eq!(
            path,
            PathBuf::from("/test/cache/example.com/path/to/file.html")
        );
    }

    #[test]
    fn get_path_with_query() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com/api?foo=bar&baz=qux")
            .expect("valid test URL");
        let path = cache.get_path(&url, Some("json"));
        assert!(path.to_string_lossy().contains("example.com"));
        assert!(path.to_string_lossy().contains("api"));
        assert!(path.to_string_lossy().ends_with(".json"));
    }

    #[test]
    fn get_path_root_url() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com").expect("valid test URL");
        let path = cache.get_path(&url, Some("html"));
        assert_eq!(path, PathBuf::from("/test/cache/example.com/__root.html"));
    }

    #[test]
    fn get_path_no_extension() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com/file").expect("valid test URL");
        let path = cache.get_path(&url, None);
        assert_eq!(path, PathBuf::from("/test/cache/example.com/file"));
    }

    #[test]
    fn exists_returns_false_for_missing_file() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com/nonexistent").expect("valid test URL");
        assert!(!cache.exists(&url, Some("html")));
    }

    #[test]
    fn get_path_with_dots_in_filename() {
        let cache = test_cache();
        let url = UrlWrapper::from_str("https://example.com/file.name.with.dots.mp3")
            .expect("valid test URL");
        let path = cache.get_path(&url, None);
        assert_eq!(
            path,
            PathBuf::from("/test/cache/example.com/file.name.with.dots.mp3")
        );
    }

    #[test]
    fn get_path_filters_empty_segments() {
        let cache = test_cache();
        let url =
            UrlWrapper::from_str("https://example.com//path///to////file").expect("valid test URL");
        let path = cache.get_path(&url, None);
        assert!(!path.to_string_lossy().contains("//"));
    }
}
