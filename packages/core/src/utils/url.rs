use crate::prelude::*;

/// Maximum allowed extension length (excluding the dot)
const MAX_EXTENSION_LENGTH: usize = 10;

pub trait UrlExtensions {
    /// Extract the file extension from the URL's path.
    ///
    /// Returns the extension (without the dot) in lowercase if:
    /// - The URL has path segments
    /// - The last path segment has a valid extension
    /// - The extension is alphanumeric and at most 10 characters
    ///
    /// Returns `None` for:
    /// - URLs without path segments
    /// - Files without extensions (e.g., `file`)
    /// - Hidden files without real extensions (e.g., `.hidden`)
    /// - Extensions with invalid characters
    /// - Extensions that are too long
    fn get_extension(&self) -> Option<String>;
}

impl UrlExtensions for Url {
    fn get_extension(&self) -> Option<String> {
        // Get the last path segment (use next_back for efficiency on DoubleEndedIterator)
        let last_segment = self.path_segments()?.next_back()?;

        // Use Path to properly extract extension from the filename
        let path = Path::new(last_segment);
        let extension = path.extension()?.to_str()?;

        // Validate extension: must be non-empty, reasonable length, and alphanumeric
        if extension.is_empty() || extension.len() > MAX_EXTENSION_LENGTH {
            return None;
        }

        if !extension.chars().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }

        Some(extension.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_extension() {
        let url = Url::parse("https://example.com/file.mp3").expect("valid url");
        assert_eq!(url.get_extension(), Some("mp3".to_owned()));
    }

    #[test]
    fn extracts_from_last_segment_only() {
        let url = Url::parse("https://example.com/path.with.dots/file.mp3").expect("valid url");
        assert_eq!(url.get_extension(), Some("mp3".to_owned()));
    }

    #[test]
    fn returns_none_for_no_extension() {
        let url = Url::parse("https://example.com/file").expect("valid url");
        assert_eq!(url.get_extension(), None);
    }

    #[test]
    fn handles_hidden_files() {
        // .hidden has no extension, it's a hidden file name
        let url = Url::parse("https://example.com/.hidden").expect("valid url");
        assert_eq!(url.get_extension(), None);
    }

    #[test]
    fn handles_double_extension() {
        // Standard behavior: extract the last extension
        let url = Url::parse("https://example.com/file.tar.gz").expect("valid url");
        assert_eq!(url.get_extension(), Some("gz".to_owned()));
    }

    #[test]
    fn lowercases_extension() {
        let url = Url::parse("https://example.com/file.MP3").expect("valid url");
        assert_eq!(url.get_extension(), Some("mp3".to_owned()));
    }

    #[test]
    fn rejects_long_extension() {
        let url = Url::parse("https://example.com/file.verylongextension").expect("valid url");
        assert_eq!(url.get_extension(), None);
    }

    #[test]
    fn rejects_non_alphanumeric_extension() {
        let url = Url::parse("https://example.com/file.mp-3").expect("valid url");
        assert_eq!(url.get_extension(), None);
    }

    #[test]
    fn handles_root_url() {
        let url = Url::parse("https://example.com/").expect("valid url");
        assert_eq!(url.get_extension(), None);
    }

    #[test]
    fn handles_query_parameters() {
        // Extension is extracted from path, not query
        let url = Url::parse("https://example.com/file.mp3?v=1.2.3").expect("valid url");
        assert_eq!(url.get_extension(), Some("mp3".to_owned()));
    }
}
