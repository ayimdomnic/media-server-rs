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
                    .table(History::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(History::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(History::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(History::MediaId).uuid().not_null())
                    .col(
                        ColumnDef::new(History::PlaybackPosition)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(timestamp(History::LastPlayedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(History::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum History {
    Table,
    Id,
    ProfileId,
    MediaId,
    PlaybackPosition,
    LastPlayedAt,
}
