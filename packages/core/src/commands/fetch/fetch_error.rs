use crate::prelude::*;

/// Errors from [`FetchHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FetchError {
    #[error("Unable to query database")]
    Repository,
    #[error("Podcast does not exist")]
    NoPodcast,
    #[error("Podcast does not have a stored feed URL")]
    NoFeedUrl,
    #[error("Unable to fetch or parse RSS feed")]
    Rss,
    #[error("Unable to save podcast")]
    Save,
    #[error("Feed redirect loop detected")]
    RedirectLoop,
    #[error("Too many feed redirects")]
    TooManyRedirects,
}

/// Errors from fetching an RSS feed.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FetchRssError {
    #[error("Unable to get feed")]
    Xml,
    #[error("An I/O error occurred")]
    Open,
    #[error("Unable to parse RSS")]
    Parse,
    #[error("Unable to convert RSS")]
    Convert,
}

/// Errors from fetching a Simplecast feed.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FetchSimplecastError {
    #[error("Unable to get page")]
    GetPage,
    #[error("Page does not contain a Simplecast Player")]
    PlayerNotFound,
    #[error("Unable to get episode")]
    GetEpisode,
    #[error("Unable to get podcast")]
    GetPodcast,
    #[error("Unable to get playlist")]
    GetPlaylist,
    #[error("Simplecast podcast does not have a feed")]
    NoFeed,
}
