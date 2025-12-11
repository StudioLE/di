use crate::prelude::*;

/// A request to execute a [`DownloadHandler`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DownloadRequest {
    pub(super) podcast: PodcastKey,
    pub(super) episode: EpisodeKey,
}

impl DownloadRequest {
    #[must_use]
    pub fn new(podcast: PodcastKey, episode: EpisodeKey) -> Self {
        Self { podcast, episode }
    }
}

impl Display for DownloadRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Podcast: {} Episode: {}", self.podcast, self.episode)
    }
}

#[cfg(feature = "server")]
impl Executable for DownloadRequest {
    type Response = ();
    type ExecutionError = DownloadError;
    type Handler = DownloadHandler;
}
