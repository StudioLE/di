use crate::prelude::*;

#[derive(Clone, Debug, Error)]
pub enum EmulateError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
    #[error("Unable to create directory")]
    CreateDirectory,
    #[error("Unable to create RSS file")]
    Create,
    #[error("Unable to write RSS file")]
    Write,
    #[error("Unable to flush RSS file")]
    Flush,
}
