use crate::prelude::*;

#[derive(Clone, Debug, Args)]
pub struct ScrapeOptions {
    /// ID of the downloaded podcast
    ///
    /// Must be alphanumeric and hyphenated
    pub podcast_slug: Slug,
    /// URL of the RSS feed or website
    pub url: Url,
}
