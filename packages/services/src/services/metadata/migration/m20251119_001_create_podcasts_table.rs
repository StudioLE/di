use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Podcasts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Podcasts::PrimaryKey)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Podcasts::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Podcasts::Title).string().not_null())
                    .col(ColumnDef::new(Podcasts::Description).text().not_null())
                    .col(ColumnDef::new(Podcasts::Image).string().null())
                    .col(ColumnDef::new(Podcasts::Language).string().null())
                    .col(ColumnDef::new(Podcasts::Categories).json().not_null())
                    .col(ColumnDef::new(Podcasts::Explicit).boolean().not_null())
                    .col(ColumnDef::new(Podcasts::Author).string().null())
                    .col(ColumnDef::new(Podcasts::Link).string().null())
                    .col(ColumnDef::new(Podcasts::Kind).string().null())
                    .col(ColumnDef::new(Podcasts::Copyright).string().null())
                    .col(ColumnDef::new(Podcasts::NewFeedUrl).string().null())
                    .col(ColumnDef::new(Podcasts::Generator).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Podcasts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Podcasts {
    Table,
    PrimaryKey,
    Slug,
    Title,
    Description,
    Image,
    Language,
    Categories,
    Explicit,
    Author,
    Link,
    Kind,
    Copyright,
    NewFeedUrl,
    Generator,
}
