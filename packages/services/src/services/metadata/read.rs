use crate::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get a [`PodcastFeed`] by its slug.
    pub async fn get_feed_by_slug(
        &self,
        slug: &str,
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
    pub async fn get_all_feeds(&self) -> Result<HashMap<String, PodcastFeed>, DbErr> {
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
        expressions.push(Expr::col((iden.clone(), episode::Column::Season)).lt(season));
    }
    expressions
        .into_iter()
        .fold(Condition::all(), Condition::add)
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[tokio::test]
    #[traced_test]
    pub async fn get_all_feeds() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_all_feeds().await;

        // Assert
        result.assert_ok_debug();
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_feed_by_slug() {
        // Arrange
        let options = None;

        // Act
        let count = _get_feed_by_slug(options).await;
        assert!(count >= 60);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_feed_by_slug__filter_year() {
        // Arrange
        let options = FilterOptions {
            year: Some(2019),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;
        assert_eq!(count, 13);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_feed_by_slug__filter_year_range() {
        // Arrange
        let options = FilterOptions {
            from_year: Some(2019),
            to_year: Some(2022),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;

        // Assert
        assert_eq!(count, 19);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_feed_by_slug__filter_season() {
        // Arrange
        let options = FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        };

        // Act
        let count = _get_feed_by_slug(Some(options)).await;

        // Assert
        assert_eq!(count, 10);
    }

    #[tokio::test]
    #[traced_test]
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
        assert_eq!(count, 17);
    }

    async fn _get_feed_by_slug(options: Option<FilterOptions>) -> usize {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_feed_by_slug("irl", options).await;

        // Assert
        let option = result.assert_ok_debug();
        let feed = option.expect("Feed should exist");
        for episode in &feed.episodes {
            println!("{episode}");
        }
        assert_eq!(feed.podcast.slug, "irl");
        feed.episodes.len()
    }
}
