use crate::prelude::*;

#[derive(Debug, Error)]
pub enum CoverError {
    #[error("Unable to get podcast")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
    #[error("Podcast does not have an image")]
    NoImage,
    #[error("Unable to get image")]
    GetImage,
    #[error("Unable to create image")]
    CreateImage,
    #[error("Unable to create directory")]
    CreateDirectory,
}
