use crate::prelude::*;

#[derive(Debug, Args)]
pub struct EmulateOptions {
    /// ID of the downloaded podcast
    ///
    /// Must be alphanumeric and hyphenated
    pub podcast_slug: Slug,
}
