use crate::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Get all podcast slugs.
    pub async fn get_all_podcast_slugs(&self) -> Result<Vec<Slug>, DbErr> {
        get_all_podcast_slugs_query()
            .into_tuple()
            .all(&self.db)
            .await
    }
}

fn get_all_podcast_slugs_query() -> Select<podcast::Entity> {
    podcast::Entity::find()
        .select_only()
        .column(podcast::Column::Slug)
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn _get_all_podcast_slugs_query() {
        // Arrange
        // Act
        let statement = get_all_podcast_slugs_query().build(DB_BACKEND);

        // Assert
        assert_snapshot!(format_sql(&statement));
    }
}
