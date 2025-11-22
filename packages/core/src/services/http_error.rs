use crate::prelude::*;
use reqwest::StatusCode;

#[allow(clippy::absolute_paths)]
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Unexpected response status: {}", get_reason(&{0}))]
    Status(u16),
    #[error("A request error occured")]
    Request,
    #[error("Unable to read cache file")]
    ReadCache,
    #[error("Unable to create cache file")]
    CreateCache,
    #[error("Unable to write cache file")]
    WriteCache,
    #[error("Unable to open cache file")]
    OpenCache,
    #[error("Unable to create cache directory")]
    CreateDirectory,
    #[error("A request error occured")]
    Chunk,
    #[error("Unable to deserialize")]
    Deserialize,
    #[error("Response did not contain a Content-Type header")]
    NoContentType,
}

fn get_reason(number: &u16) -> &str {
    StatusCode::from_u16(*number)
        .map(|e| e.canonical_reason())
        .ok()
        .flatten()
        .unwrap_or_default()
}
