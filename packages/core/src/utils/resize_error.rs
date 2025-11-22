use crate::prelude::*;

#[derive(Clone, Debug, Error)]
pub enum ResizeError {
    #[error("Unable to open image")]
    Open,
    #[error("Unable to determine image format")]
    Format,
    #[error("Unable to decode image")]
    Decode,
    #[error("Unable to encode image")]
    Encode,
    #[error("Unable to write image to file")]
    Write,
    #[error("Unable to resize image")]
    Resize,
}
