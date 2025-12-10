use crate::prelude::*;
use sea_orm::Selector;
use sea_orm::*;

impl MetadataRepository {
    /// Get a podcast with minimal info for the podcast page.
    pub async fn get_podcast(
        &self,
        slug: Slug,
    ) -> Result<Option<(PodcastPartial, Vec<EpisodePartial>)>, DbErr> {
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

fn get_podcast_query(slug: Slug) -> Selector<SelectModel<PodcastPartial>> {
    podcast::Entity::find_by_slug(slug)
        .join(JoinType::LeftJoin, podcast::Relation::Episode.def())
        .select_only()
        .columns([
            podcast::Column::PrimaryKey,
            podcast::Column::Slug,
            podcast::Column::Title,
            podcast::Column::Image,
        ])
        .expr_as(episode::Column::PrimaryKey.count(), "episodes_count")
        .into_model::<PodcastPartial>()
}

fn get_episodes_query(primary_key: u32) -> Selector<SelectModel<EpisodePartial>> {
    episode::Entity::find()
        .has_related(podcast::Entity, podcast::Column::PrimaryKey.eq(primary_key))
        .select_only()
        .columns([
            episode::Column::PrimaryKey,
            episode::Column::Title,
            episode::Column::PublishedAt,
            episode::Column::SourceDuration,
            episode::Column::Image,
            episode::Column::Episode,
            episode::Column::Season,
            episode::Column::Kind,
        ])
        .order_by_asc(episode::Column::PublishedAt)
        .into_model::<EpisodePartial>()
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    pub fn _get_podcast_query() {
        // Arrange
        let slug = example_slug();

        // Act
        let statement = get_podcast_query(slug).into_statement(DB_BACKEND);

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
    pub async fn get_podcast() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let slug = MetadataRepositoryExample::podcast_slug();
        let _logger = init_test_logger();

        // Act
        let result = metadata.get_podcast(slug).await;

        // Assert
        let (podcast, episodes) = result.assert_ok_debug().expect("Podcast should exist");
        assert_yaml_snapshot!((podcast, episodes));
    }
}
