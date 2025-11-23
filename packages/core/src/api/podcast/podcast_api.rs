use crate::prelude::*;
use sea_orm::Selector;
use sea_orm::*;

impl MetadataRepository {
    /// Get a podcast with minimal info for the podcast page.
    pub async fn get_podcast(
        &self,
        slug: &str,
    ) -> Result<Option<(PodcastPagePartial, Vec<PodcastPageEpisodePartial>)>, DbErr> {
        let option = get_podcast_query(slug).one(&self.db).await?;
        let Some(podcast) = option else {
            return Ok(None);
        };
        let episodes = get_episodes_query(podcast.primary_key)
            .all(&self.db)
            .await?;
        Ok(Some((podcast, episodes)))
    }
}

fn get_podcast_query(slug: &str) -> Selector<SelectModel<PodcastPagePartial>> {
    podcast::Entity::find_by_slug(slug).into_partial_model::<PodcastPagePartial>()
}

fn get_episodes_query(primary_key: u32) -> Selector<SelectModel<PodcastPageEpisodePartial>> {
    episode::Entity::find()
        .has_related(
            podcast::Entity,
            podcast::Column::PrimaryKey.eq(primary_key),
        )
        .order_by_asc(episode::Column::PublishedAt)
        .into_partial_model::<PodcastPageEpisodePartial>()
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    pub fn _get_podcast_query() {
        // Arrange
        // Act
        let statement = get_podcast_query(PODCAST_SLUG).into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[test]
    pub fn _get_episodes_query() {
        // Arrange
        // Act
        let statement = get_episodes_query(PODCAST_KEY).into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_podcast() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_podcast("irl").await;

        // Assert
        let (podcast, episodes) = result.assert_ok_debug().expect("Podcast should exist");
        assert_yaml_snapshot!(podcast);
        assert_yaml_snapshot!(episodes);
    }
}
