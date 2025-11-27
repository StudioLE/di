use crate::prelude::*;

#[derive(Debug, Args)]
pub struct DownloadOptions {
    /// ID of the downloaded podcast
    ///
    /// Must be alphanumeric and hyphenated
    pub podcast_slug: Slug,
    #[command(flatten)]
    pub filter: FilterOptions,
}
