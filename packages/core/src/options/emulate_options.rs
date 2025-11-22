use crate::prelude::*;

#[derive(Debug, Args)]
pub struct EmulateOptions {
    /// ID of the downloaded podcast
    ///
    /// Must be alphanumeric and hyphenated
    #[arg(value_parser = Validator::validate_id)]
    pub podcast_slug: String,
}
