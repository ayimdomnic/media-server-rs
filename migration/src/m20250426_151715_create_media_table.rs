use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Media::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Media::LibraryId).uuid().not_null())
                    .col(string(Media::Title).not_null())
                    .col(string(Media::FilePath).not_null())
                    .col(string(Media::MediaType).not_null()) // e.g., "audio", "video", "image"
                    .col(timestamp(Media::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Media::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Media {
    Table,
    Id,
    Title,
    FilePath,
    MediaType,
    LibraryId,
    CreatedAt,
    UpdatedAt,
}
