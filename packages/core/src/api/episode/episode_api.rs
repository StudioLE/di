use crate::prelude::*;
use sea_orm::Selector;
use sea_orm::*;

impl MetadataRepository {
    /// Get an episode with minimal info for the episode page.
    pub async fn get_episode(
        &self,
        podcast_slug: &str,
        episode_key: u32,
    ) -> Result<Option<EpisodePagePartial>, DbErr> {
        get_episode_query(podcast_slug, episode_key)
            .one(&self.db)
            .await
    }
}

fn get_episode_query(
    podcast_slug: &str,
    episode_key: u32,
) -> Selector<SelectModel<EpisodePagePartial>> {
    episode::Entity::find_by_id(episode_key)
        .join(JoinType::InnerJoin, episode::Relation::Podcast.def())
        .filter(podcast::Column::Slug.eq(podcast_slug))
        .into_partial_model::<EpisodePagePartial>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn _get_episode_query() {
        // Arrange
        // Act
        let statement = get_episode_query(PODCAST_SLUG, EPISODE_KEY).into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    #[traced_test]
    #[ignore = "Requires an unmodified db"]
    pub async fn get_episode() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services
            .metadata
            .get_episode(PODCAST_SLUG, EPISODE_KEY)
            .await;

        // Assert
        let episode = result.assert_ok_debug().expect("Episode should exist");
        assert_yaml_snapshot!(episode);
    }
}
