use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get a [`PodcastFeed`] by its slug.
    pub async fn get_feed_by_slug(
        &self,
        slug: Slug,
        options: Option<FilterOptions>,
    ) -> Result<Option<PodcastFeed>, DbErr> {
        let query = podcast::Entity::find_by_slug(slug);
        let query = if let Some(options) = options {
            query
                .join_as(
                    JoinType::LeftJoin,
                    podcast::Relation::Episode
                        .def()
                        .on_condition(move |_left, right| {
                            get_filter_condition(right, options.clone())
                        }),
                    episode::Entity,
                )
                .select_with(episode::Entity)
        } else {
            query.find_with_related(episode::Entity)
        };
        let feeds = query.all(&self.db).await?;
        Ok(feeds.into_iter().next().map(PodcastFeed::from))
    }

    /// Get all [`PodcastFeed`].
    pub async fn get_all_feeds(&self) -> Result<HashMap<Slug, PodcastFeed>, DbErr> {
        let feeds = podcast::Entity::find()
            .find_with_related(episode::Entity)
            .all(&self.db)
            .await?
            .into_iter()
            .map(PodcastFeed::from)
            .map(|feed| (feed.podcast.slug.clone(), feed))
            .collect();
        Ok(feeds)
    }
}

fn get_filter_condition(iden: DynIden, options: FilterOptions) -> Condition {
    let mut expressions = Vec::new();
    if let Some(year) = options.year {
        expressions
            .push(Expr::col((iden.clone(), episode::Column::PublishedAt)).like(format!("{year}%")));
    }
    if let Some(year) = options.from_year {
        expressions.push(
            Expr::col((iden.clone(), episode::Column::PublishedAt)).gte(format!("{year}-01-01")),
        );
    }
    if let Some(year) = options.to_year {
        expressions.push(
            Expr::col((iden.clone(), episode::Column::PublishedAt))
                .lt(format!("{}-01-01", year + 1)),
        );
    }
    if let Some(season) = options.season {
        expressions
            .push(Expr::col((iden.clone(), episode::Column::Season)).like(format!("{season}%")));
    }
    if let Some(season) = options.from_season {
        expressions.push(Expr::col((iden.clone(), episode::Column::PublishedAt)).gte(season));
    }
    if let Some(season) = options.to_season {
        expressions.push(Expr::col((iden.clone(), episode::Column::Season)).lte(season));
    }
    expressions
        .into_iter()
        .fold(Condition::all(), Condition::add)
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case, clippy::as_conversions, clippy::cast_possible_wrap)]
    use super::*;

    #[tokio::test]
    pub async fn get_all_feeds() {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let _logger = init_test_logger();

        // Act
        let result = metadata.get_all_feeds().await;

        // Assert
        let hash_map = result.assert_ok_debug();
        assert_eq!(
            hash_map.keys().len(),
            MetadataRepositoryExample::PODCAST_COUNT,
            "podcast count"
        );
    }

    #[tokio::test]
    pub async fn get_feed_by_slug() {
        // Arrange
        let options = None;

        // Act
        let count = _get_feed_by_slug(options).await;
        let expected = MetadataRepositoryExample::SEASONS_PER_YEAR
            * MetadataRepositoryExample::EPISODES_PER_SEASON
            * MetadataRepositoryExample::YEAR_COUNT;
        assert_eq!(count, expected as usize);
    }

    #[tokio::test]
    pub async fn get_feed_by_slug__filter_year() {
        // Arrange
        let options = FilterOptions {
            year: Some(MetadataRepositoryExample::START_YEAR as i32),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;
        let expected = MetadataRepositoryExample::SEASONS_PER_YEAR
            * MetadataRepositoryExample::EPISODES_PER_SEASON;
        assert_eq!(count, expected as usize);
    }

    #[tokio::test]
    pub async fn get_feed_by_slug__filter_year_range() {
        // Arrange
        let options = FilterOptions {
            from_year: Some(MetadataRepositoryExample::START_YEAR as i32),
            to_year: Some(MetadataRepositoryExample::START_YEAR as i32 + 2),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;

        // Assert
        let expected = 3
            * MetadataRepositoryExample::SEASONS_PER_YEAR
            * MetadataRepositoryExample::EPISODES_PER_SEASON;
        assert_eq!(count, expected as usize);
    }

    #[tokio::test]
    pub async fn get_feed_by_slug__filter_season() {
        // Arrange
        let options = FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;

        // Assert
        assert_eq!(
            count,
            MetadataRepositoryExample::EPISODES_PER_SEASON as usize
        );
    }

    #[tokio::test]
    pub async fn get_feed_by_slug__filter_season_range() {
        // Arrange
        let options = FilterOptions {
            from_season: Some(1),
            to_season: Some(3),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;

        // Assert
        let expected = 3 * MetadataRepositoryExample::EPISODES_PER_SEASON;
        assert_eq!(count, expected as usize);
    }

    async fn _get_feed_by_slug(options: Option<FilterOptions>) -> usize {
        // Arrange
        let metadata = MetadataRepositoryExample::create().await;
        let slug = MetadataRepositoryExample::podcast_slug();
        let _logger = init_test_logger();

        // Act
        let result = metadata.get_feed_by_slug(slug.clone(), options).await;

        // Assert
        let option = result.assert_ok_debug();
        let feed = option.expect("Feed should exist");
        for episode in &feed.episodes {
            println!("{episode}");
        }
        assert_eq!(feed.podcast.slug, slug);
        feed.episodes.len()
    }
}
