use crate::prelude::*;
use sea_orm::Selector;
use sea_orm::*;

impl MetadataRepository {
    /// Get all podcasts with minimal info for the index page.
    pub async fn get_podcasts(&self) -> Result<Vec<IndexPagePodcastPartial>, DbErr> {
        let podcasts = get_podcasts_query().all(&self.db).await?;
        Ok(podcasts)
    }
}

fn get_podcasts_query() -> Selector<SelectModel<IndexPagePodcastPartial>> {
    podcast::Entity::find()
        .order_by_asc(podcast::Column::Title)
        .join(JoinType::LeftJoin, podcast::Relation::Episode.def())
        .group_by(podcast::Column::PrimaryKey)
        .into_partial_model::<IndexPagePodcastPartial>()
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    pub fn _get_podcasts_query() {
        // Arrange
        // Act
        let statement = get_podcasts_query().into_statement(DB_BACKEND);

        // Assert
        let sql = format_sql(&statement);
        assert_snapshot!(sql);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn get_podcasts() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");

        // Act
        let result = services.metadata.get_podcasts().await;

        // Assert
        let podcasts = result.assert_ok_debug();
        assert_yaml_snapshot!(podcasts);
    }
}
