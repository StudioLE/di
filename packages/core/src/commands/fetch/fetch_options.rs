use crate::prelude::*;

/// CLI options for [`FetchCliCommand`].
#[derive(Clone, Debug, Args)]
pub struct FetchOptions {
    /// Slug of the podcast to fetch (omit to fetch all podcasts).
    #[arg(long)]
    pub podcast: Option<Slug>,
}
