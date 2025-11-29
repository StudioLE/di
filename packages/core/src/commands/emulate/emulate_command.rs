use super::to_rss::PodcastToRss;
use crate::prelude::*;
use rss::Item as RssItem;

pub struct EmulateCommand {
    paths: Arc<PathProvider>,
    metadata: Arc<MetadataRepository>,
}

impl Service for EmulateCommand {
    type Error = ServiceError;
    async fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(
            services.get_service().await?,
            services.get_service().await?,
        ))
    }
}

impl EmulateCommand {
    #[must_use]
    pub fn new(paths: Arc<PathProvider>, metadata: Arc<MetadataRepository>) -> Self {
        Self { paths, metadata }
    }

    pub async fn execute(&self, options: EmulateOptions) -> Result<(), Report<EmulateError>> {
        // TODO: Add support for filtering episodes.
        let feed = self
            .metadata
            .get_feed_by_slug(options.podcast_slug, None)
            .await
            .change_context(EmulateError::Repository)?
            .ok_or(EmulateError::NoPodcast)?;
        let feeds = self.save_feeds(&feed).await?;
        info!("Created {} rss feeds", feeds.len());
        Ok(())
    }

    async fn save_feeds(&self, feed: &PodcastFeed) -> Result<Vec<PathBuf>, Report<EmulateError>> {
        let mut paths = Vec::new();
        paths.push(self.save_feed(feed, None, None).await?);
        let mut feed = feed.clone();
        let groups = group_by_season(take(&mut feed.episodes));
        for (season, episodes) in groups {
            let mut p = feed.clone();
            p.episodes = episodes;
            paths.push(self.save_feed(&p, season, None).await?);
            let year_groups = group_by_year(take(&mut p.episodes));
            for (year, episodes) in year_groups {
                p.episodes = episodes;
                paths.push(self.save_feed(&p, season, Some(year)).await?);
            }
        }
        Ok(paths)
    }

    async fn save_feed(
        &self,
        feed: &PodcastFeed,
        season: Option<u32>,
        year: Option<i32>,
    ) -> Result<PathBuf, Report<EmulateError>> {
        let mut channel = PodcastToRss::execute(feed.clone());
        for item in &mut channel.items {
            self.replace_enclosure(feed, item);
        }
        let xml = channel.to_string();
        let path = self.paths.get_rss_path(&feed.podcast.slug, season, year);
        create_parent_dir_if_not_exist(&path)
            .await
            .change_context(EmulateError::CreateDirectory)?;
        let mut file = AsyncFile::create(&path)
            .await
            .change_context(EmulateError::Create)
            .attach_path(&path)?;
        file.write_all(xml.as_bytes())
            .await
            .change_context(EmulateError::Write)
            .attach_path(&path)?;
        file.flush()
            .await
            .change_context(EmulateError::Flush)
            .attach_path(&path)?;
        Ok(path)
    }

    fn replace_enclosure(&self, feed: &PodcastFeed, item: &mut RssItem) -> Option<()> {
        let guid = item.guid.clone()?;
        let episode = feed
            .episodes
            .iter()
            .find(|episode| episode.source_id == guid.value)?;
        let enclosure = item.enclosure.as_mut()?;
        enclosure.url = self
            .paths
            .get_audio_url(&feed.podcast.slug, episode)?
            .to_string();
        Some(())
    }
}

fn group_by_season(episodes: Vec<EpisodeInfo>) -> HashMap<Option<u32>, Vec<EpisodeInfo>> {
    let mut groups: HashMap<Option<u32>, Vec<EpisodeInfo>> = HashMap::new();
    for episode in episodes {
        let group = groups.entry(episode.season).or_default();
        group.push(episode);
    }
    groups
}

fn group_by_year(episodes: Vec<EpisodeInfo>) -> HashMap<i32, Vec<EpisodeInfo>> {
    let mut groups: HashMap<i32, Vec<EpisodeInfo>> = HashMap::new();
    for episode in episodes {
        let year = episode.published_at.year();
        let group = groups.entry(year).or_default();
        group.push(episode);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn feeds_command() {
        // Arrange
        let services = ServiceProvider::new();
        let command = services
            .get_service::<EmulateCommand>()
            .await
            .expect("should be able to get command");
        let options = EmulateOptions {
            podcast_slug: example_slug(),
        };

        // Act
        let result = command.execute(options).await;

        // Assert
        result.assert_ok_debug();
    }
}
