use crate::prelude::*;

#[derive(Clone, Debug, Args)]
pub struct ScrapeOptions {
    /// ID of the downloaded podcast
    ///
    /// Must be alphanumeric and hyphenated
    #[arg(value_parser = Validator::validate_id)]
    pub podcast_slug: String,
    /// URL of the RSS feed or website
    pub url: Url,
}
