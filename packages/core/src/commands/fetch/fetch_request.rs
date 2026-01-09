use crate::prelude::*;

/// A request to execute a [`FetchHandler`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FetchRequest {
    /// User-defined identifier for the podcast.
    pub slug: Slug,
}

impl Display for FetchRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "fetch {}", self.slug)
    }
}

impl Executable for FetchRequest {
    type Response = FetchResponse;
    type ExecutionError = Report<FetchError>;
}
