use crate::prelude::*;

#[derive(Clone, Debug, Error)]
pub enum GetMetadataError {
    #[error("Podcast not found")]
    NotFound,
    #[error("Unable to open file")]
    Open,
    #[error("Unable to deserialize file")]
    Deserialize,
}

#[derive(Clone, Debug, Error)]
pub enum PutMetadataError {
    #[error("Unable to create file")]
    Create,
    #[error("Unable to serialize file")]
    Serialize,
}
