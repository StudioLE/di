use crate::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::*;

impl MetadataRepository {
    /// Save a the [`PodcastInfo`] and [`EpisodeInfo`] entities.
    ///
    /// If a podcast with the same slug already exists it will be overwritten.
    #[allow(clippy::single_match_else)]
    pub async fn save_feed(&self, feed: PodcastFeed) -> Result<PodcastFeed, Report<SaveError>> {
        let tx = self.db.begin().await.change_context(SaveError::Begin)?;
        let primary_key = match get_podcast_key_by_slug(&tx, &feed.podcast.slug).await? {
            Some(key) => {
                trace!(
                    podcast = feed.podcast.slug,
                    key, "Overwriting existing podcast"
                );
                remove_podcast(&tx, key).await?;
                Set(key)
            }
            None => {
                trace!(podcast = feed.podcast.slug, "Inserting new podcast");
                NotSet
            }
        };
        let model = podcast::ActiveModel {
            primary_key,
            ..podcast::ActiveModel::from(feed.podcast)
        };
        let podcast = model.insert(&tx).await.change_context(SaveError::Podcast)?;
        let models = feed
            .episodes
            .into_iter()
            .map(|episode| episode::ActiveModel {
                primary_key: NotSet,
                podcast_key: Set(Some(podcast.primary_key)),
                ..episode::ActiveModel::from(episode)
            });
        let episodes = episode::Entity::insert_many(models)
            .exec_with_returning(&tx)
            .await
            .change_context(SaveError::Episodes)?;
        tx.commit().await.change_context(SaveError::Commit)?;
        Ok(PodcastFeed { podcast, episodes })
    }
}

async fn remove_podcast(
    tx: &DatabaseTransaction,
    primary_key: u32,
) -> Result<DeleteResult, Report<SaveError>> {
    podcast::Entity::delete_by_id(primary_key)
        .exec(tx)
        .await
        .change_context(SaveError::Commit)
}

/// Check if a podcast with the given slug already exists
async fn get_podcast_key_by_slug(
    tx: &DatabaseTransaction,
    slug: &str,
) -> Result<Option<u32>, Report<SaveError>> {
    let key = podcast::Entity::find()
        .select_only()
        .column(podcast::Column::PrimaryKey)
        .filter(podcast::Column::Slug.eq(slug))
        .into_tuple::<u32>()
        .one(tx)
        .await
        .change_context(SaveError::Unique)?;
    Ok(key)
}

#[derive(Clone, Debug, Error)]
pub enum SaveError {
    #[error("Unable to begin database transaction")]
    Begin,
    #[error("Unable to check if podcast already exists")]
    Unique,
    #[error("Unable to remove previous podcast")]
    Remove,
    #[error("Unable to insert podcast")]
    Podcast,
    #[error("Unable to insert episodes")]
    Episodes,
    #[error("Unable to commit database transaction")]
    Commit,
}
