use crate::prelude::*;

#[derive(Clone, Debug, Error)]
pub enum ScrapeError {
    #[error("Unable to get content type")]
    Head,
    #[error("Unable to get RSS feed from simplecast player")]
    Simplecast,
    #[error("Unable to get RSS feed")]
    Rss,
    #[error("Unable to save")]
    Save,
}

#[derive(Clone, Debug, Error)]
pub enum ScrapeRssError {
    #[error("Unable to get feed")]
    Xml,
    #[error("An I/O error occurred")]
    Open,
    #[error("Unable to parse RSS")]
    Parse,
    #[error("Unable to convert RSS")]
    Convert,
}

#[derive(Clone, Debug, Error)]
pub enum ScrapeSimplecastError {
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
