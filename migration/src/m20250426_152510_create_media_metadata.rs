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
                    .table(MediaMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MediaMetadata::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MediaMetadata::MediaId).uuid().not_null())
                    .col(ColumnDef::new(MediaMetadata::Metadata).json_binary().null()) // Flexible JSON for metadata
                    .col(timestamp(MediaMetadata::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(MediaMetadata::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(MediaMetadata::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MediaMetadata {
    Table,
    Id,
    MediaId,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
