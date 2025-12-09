use crate::prelude::*;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum EmulateError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
    #[error("Episode has not been downloaded")]
    NoPath,
    #[error("Server base option must be set")]
    NoServerBase,
    #[error("Unable to parse URL")]
    ParseUrl,
    #[error("Episode does not have a GUID")]
    NoGuid,
    #[error("Unable to match episode to RSS channel item")]
    NoMatch,
    #[error("Episode does not have an enclosure URL")]
    NoEnclosure,
    #[error("Unable to create directory")]
    CreateDirectory,
    #[error("Unable to create RSS file")]
    Create,
    #[error("Unable to write RSS file")]
    Write,
    #[error("Unable to flush RSS file")]
    Flush,
}
