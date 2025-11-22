use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Episodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Episodes::PrimaryKey)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Episodes::PodcastKey).unsigned().null())
                    .col(ColumnDef::new(Episodes::SourceId).string().not_null())
                    .col(ColumnDef::new(Episodes::Title).string().not_null())
                    .col(ColumnDef::new(Episodes::SourceUrl).string().not_null())
                    .col(
                        ColumnDef::new(Episodes::SourceFileSize)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Episodes::SourceContentType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Episodes::PublishedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Episodes::Description).text().null())
                    .col(ColumnDef::new(Episodes::SourceDuration).unsigned().null())
                    .col(ColumnDef::new(Episodes::Image).string().null())
                    .col(ColumnDef::new(Episodes::Explicit).boolean().null())
                    .col(ColumnDef::new(Episodes::ItunesTitle).string().null())
                    .col(ColumnDef::new(Episodes::Episode).unsigned().null())
                    .col(ColumnDef::new(Episodes::Season).unsigned().null())
                    .col(ColumnDef::new(Episodes::Kind).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_episodes_podcast_key")
                            .from(Episodes::Table, Episodes::PodcastKey)
                            .to(Podcasts::Table, Podcasts::PrimaryKey)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Episodes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Episodes {
    Table,
    PrimaryKey,
    PodcastKey,
    SourceId,
    Title,
    SourceUrl,
    SourceFileSize,
    SourceContentType,
    PublishedAt,
    Description,
    SourceDuration,
    Image,
    Explicit,
    ItunesTitle,
    Episode,
    Season,
    Kind,
}

#[derive(DeriveIden)]
enum Podcasts {
    Table,
    PrimaryKey,
}
