use crate::prelude::*;
use error_stack::{FutureExt, ResultExt};

#[allow(dead_code)]
const CONCURRENCY: usize = 8;

impl ScrapeCommand {
    pub(super) async fn get_simplecast_rss(
        &self,
        options: &ScrapeOptions,
    ) -> Result<Url, Report<ScrapeSimplecastError>> {
        let player_id = self.get_player_id(&options.url).await?;
        let episode = self.get_episode(&player_id).await?;
        let podcast = self.get_podcast(&episode).await?;
        if let Some(url) = podcast.feed_url {
            return Ok(url);
        }
        if let Some(url) = podcast.external_feed_url {
            return Ok(url);
        }
        let report = Report::new(ScrapeSimplecastError::NoFeed)
            .attach(format!("Podcast ID: {}", podcast.id));
        Err(report)
    }

    async fn get_player_id(&self, url: &Url) -> Result<String, Report<ScrapeSimplecastError>> {
        let html = self
            .http
            .get_html(url)
            .await
            .change_context(ScrapeSimplecastError::GetPage)
            .attach_url(url)?;
        let episode_guid = get_simplecast_episode_guid(&html).ok_or_else(|| {
            Report::new(ScrapeSimplecastError::PlayerNotFound).attach(format!("URL: {url}"))
        })?;
        trace!("Found Simplecast player with episode id: {episode_guid}",);
        Ok(episode_guid)
    }

    async fn get_episode(
        &self,
        id: &str,
    ) -> Result<SimplecastEpisode, Report<ScrapeSimplecastError>> {
        let episode_url = Url::parse(&format!("https://api.simplecast.com/episodes/{id}"))
            .expect("URL should be valid");
        let episode: SimplecastEpisode = self
            .http
            .get_json(&episode_url)
            .await
            .change_context(ScrapeSimplecastError::GetEpisode)
            .attach_with(|| format!("Episode ID: {id}"))?;
        Ok(episode)
    }

    async fn get_podcast(
        &self,
        episode: &SimplecastEpisode,
    ) -> Result<SimplecastPodcast, Report<ScrapeSimplecastError>> {
        debug!("Fetching podcast for {}", episode.podcast.title);
        let url = Url::parse(&format!(
            "https://api.simplecast.com/podcasts/{}",
            episode.podcast.id
        ))
        .expect("URL should be valid");
        self.http
            .get_json(&url)
            .await
            .change_context(ScrapeSimplecastError::GetPodcast)
            .attach_with(|| format!("Podcast ID: {}", episode.podcast.id))
    }

    #[allow(dead_code)]
    async fn get_playlist(
        &self,
        episode: &SimplecastEpisode,
    ) -> Result<Vec<SimplecastPlaylistEpisode>, Report<ScrapeSimplecastError>> {
        debug!("Fetching playlist for {}", episode.podcast.title);
        let mut playlist_url = Url::parse(&format!(
            "https://api.simplecast.com/podcasts/{}/playlist",
            episode.podcast.id
        ))
        .expect("URL should be valid");
        let mut episodes = Vec::new();
        loop {
            let mut playlist: SimplecastPlaylist = self
                .http
                .get_json(&playlist_url)
                .await
                .change_context(ScrapeSimplecastError::GetPlaylist)
                .attach_with(|| format!("Podcast ID: {}", episode.podcast.id))?;
            let next = playlist.episodes.pages.next.clone();
            episodes.append(&mut playlist.episodes.collection);
            let Some(link) = next else {
                break;
            };
            playlist_url = link.href;
        }
        Ok(episodes)
    }

    #[allow(dead_code)]
    async fn get_episodes(
        &self,
        playlist: &[SimplecastPlaylistEpisode],
    ) -> Result<Vec<SimplecastEpisode>, Report<ScrapeSimplecastError>> {
        debug!("Fetching metadata for {} episodes", playlist.len());
        stream::iter(playlist.iter().map(|episode| {
            let this = self;
            async move { this.get_episode(&episode.id).await }
        }))
        .buffer_unordered(CONCURRENCY)
        .try_collect::<Vec<_>>()
        .await
    }
}

fn get_simplecast_episode_guid(html: &Html) -> Option<String> {
    let mut src = get_element_attr(html, "iframe", "src");
    src.append(&mut get_element_attr(html, "iframe", "data-src"));
    src.into_iter().find_map(|url| {
        if url.is_empty() {
            return None;
        }
        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(e) => {
                warn!(url, %e, "Unable to parse URL");
                return None;
            }
        };
        let host = url.host_str()?;
        if host != "player.simplecast.com" && host != "embed.simplecast.com" {
            return None;
        }
        let guid = url.path_segments()?.next()?.to_owned();
        Some(guid)
    })
}

fn get_element_attr(html: &Html, selector: &str, attr: &str) -> Vec<String> {
    html.select(&Selector::parse(selector).expect("Selector should be valid"))
        .filter_map(|element| element.attr(attr).map(str::to_owned))
        .collect()
}

#[allow(dead_code)]
fn convert(
    podcast_id: &str,
    podcast: SimplecastPodcast,
    episodes: Vec<SimplecastEpisode>,
) -> Podcast {
    let mut podcast: Podcast = podcast.into();
    podcast_id.clone_into(&mut podcast.id);
    podcast.episodes = episodes.into_iter().map(Into::into).collect();
    podcast
}
