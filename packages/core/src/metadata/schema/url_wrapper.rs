use crate::prelude::*;
use sea_orm::sea_query::*;
use sea_orm::*;

/// A validated URL wrapper for `SeaORM` model fields.
///
/// Validates URLs on construction via `FromStr`:
/// - Scheme must be `http` or `https`
/// - No path traversal sequences (`.` or `..`)
/// - No null bytes or control characters
/// - Path segments within 255 bytes
///
/// See <https://www.sea-ql.org/SeaORM/docs/generate-entity/newtype/>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UrlWrapper(Url);

// Convenience traits
impl Display for UrlWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UrlWrapper {
    type Err = Report<UrlError>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(value).change_context(UrlError::Parse)?;
        validate_url(&url).attach(format!("URL: {url}"))?;
        Ok(UrlWrapper(url))
    }
}

impl AsRef<Url> for UrlWrapper {
    fn as_ref(&self) -> &Url {
        &self.0
    }
}

impl Deref for UrlWrapper {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SeaORM traits

impl From<UrlWrapper> for Value {
    fn from(url: UrlWrapper) -> Self {
        Value::String(Some(url.0.to_string()))
    }
}

impl TryGetable for UrlWrapper {
    fn try_get_by<I: ColIdx>(res: &QueryResult, index: I) -> Result<Self, TryGetError> {
        let value: Option<String> = res.try_get_by(index).map_err(TryGetError::DbErr)?;
        match value {
            Some(s) => Url::from_str(&s)
                .map(UrlWrapper)
                .map_err(|e| TryGetError::DbErr(DbErr::Type(format!("Invalid URL: {e}")))),
            None => Err(TryGetError::Null(format!("{index:?}"))),
        }
    }
}

impl ValueType for UrlWrapper {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::String(Some(s)) => Url::from_str(&s).map(UrlWrapper).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        "UrlWrapper".to_owned()
    }

    fn array_type() -> ArrayType {
        ArrayType::String
    }

    fn column_type() -> ColumnType {
        ColumnType::String(StringLen::default())
    }
}

impl Nullable for UrlWrapper {
    fn null() -> Value {
        Value::String(None)
    }
}

// Validation

const ALLOWED_SCHEMES: &[&str] = &["http", "https"];
const MAX_SEGMENT_LENGTH: usize = 255;

/// Errors that can occur when validating a URL for safe HTTP operations.
#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Not a URL")]
    Parse,
    #[error("URL scheme must be either HTTP or HTTPS")]
    NotHttp,
    #[error("Path traversal detected")]
    PathTraversal,
    #[error("Path segment contains null byte")]
    NullByte,
    #[error("Path segment contains control character")]
    ControlCharacter,
    #[error("Path segment too long (max {MAX_SEGMENT_LENGTH} bytes)")]
    SegmentTooLong,
    #[error("Invalid domain - contains path separator")]
    InvalidDomain,
}

fn validate_url(url: &Url) -> Result<(), Report<UrlError>> {
    validate_scheme(url)?;
    validate_domain(url)?;
    validate_path_segments(url)?;
    Ok(())
}

fn validate_scheme(url: &Url) -> Result<(), Report<UrlError>> {
    let scheme = url.scheme();
    if !ALLOWED_SCHEMES.contains(&scheme) {
        return Err(Report::new(UrlError::NotHttp).attach(format!("Scheme: {scheme}")));
    }
    Ok(())
}

fn validate_domain(url: &Url) -> Result<(), Report<UrlError>> {
    if let Some(domain) = url.domain()
        && (domain.contains('/') || domain.contains('\\') || domain.contains('\0'))
    {
        return Err(Report::new(UrlError::InvalidDomain).attach(format!("Domain: {domain}")));
    }
    Ok(())
}

fn validate_path_segments(url: &Url) -> Result<(), Report<UrlError>> {
    let Some(segments) = url.path_segments() else {
        return Ok(());
    };
    for segment in segments {
        validate_segment(segment).attach(format!("Segment: {segment}"))?;
    }
    Ok(())
}

fn validate_segment(segment: &str) -> Result<(), Report<UrlError>> {
    if segment == "." || segment == ".." {
        return Err(Report::new(UrlError::PathTraversal));
    }
    if segment.contains('\0') {
        return Err(Report::new(UrlError::NullByte));
    }
    if segment.bytes().any(|b| b < 0x20) {
        return Err(Report::new(UrlError::ControlCharacter));
    }
    if segment.len() > MAX_SEGMENT_LENGTH {
        return Err(
            Report::new(UrlError::SegmentTooLong).attach(format!("Length: {}", segment.len()))
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::discriminant;

    fn is_error<T>(result: Result<T, Report<UrlError>>, expected: UrlError) -> bool {
        let Err(report) = result else { return false };
        discriminant(report.current_context()) == discriminant(&expected)
    }

    #[test]
    fn accepts_http() {
        let result = UrlWrapper::from_str("http://example.com/path");
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_https() {
        let result = UrlWrapper::from_str("https://example.com/path");
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_file_scheme() {
        let result = UrlWrapper::from_str("file:///etc/passwd");
        assert!(is_error(result, UrlError::NotHttp));
    }

    #[test]
    fn rejects_ftp_scheme() {
        let result = UrlWrapper::from_str("ftp://example.com/file");
        assert!(is_error(result, UrlError::NotHttp));
    }

    #[test]
    fn rejects_data_scheme() {
        let result = UrlWrapper::from_str("data:text/plain,hello");
        assert!(is_error(result, UrlError::NotHttp));
    }

    #[test]
    fn url_crate_normalizes_path_traversal() {
        // The url crate normalizes `..` and `.` per RFC 3986
        // Our validation is defense-in-depth for edge cases
        let url = UrlWrapper::from_str("https://example.com/../etc/passwd")
            .expect("normalized URL should be valid");
        assert_eq!(url.path(), "/etc/passwd");
    }

    #[test]
    fn validate_segment_rejects_path_traversal() {
        // Direct segment validation catches traversal attempts
        assert!(is_error(validate_segment(".."), UrlError::PathTraversal));
        assert!(is_error(validate_segment("."), UrlError::PathTraversal));
    }

    #[test]
    fn rejects_segment_too_long() {
        let long_segment = "a".repeat(256);
        let url = format!("https://example.com/{long_segment}");
        let result = UrlWrapper::from_str(&url);
        assert!(is_error(result, UrlError::SegmentTooLong));
    }

    #[test]
    fn accepts_segment_at_limit() {
        let segment = "a".repeat(255);
        let url = format!("https://example.com/{segment}");
        let result = UrlWrapper::from_str(&url);
        assert!(result.is_ok());
    }

    #[test]
    fn accepts_valid_url_with_query() {
        let result = UrlWrapper::from_str("https://example.com/path?foo=bar&baz=123");
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_url() {
        let result = UrlWrapper::from_str("not a url");
        assert!(is_error(result, UrlError::Parse));
    }
}
