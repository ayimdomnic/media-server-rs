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
                    .table(UserActivity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserActivity::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserActivity::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(UserActivity::MediaId).uuid().not_null())
                    .col(string(UserActivity::ActivityType).not_null()) // e.g., "play", "pause", "seek", "rate"
                    .col(
                        ColumnDef::new(UserActivity::ActivityData)
                            .json_binary()
                            .null(), // Flexible JSON for activity-specific data
                    ) // Flexible JSON for activity-specific data
                    .col(timestamp(UserActivity::Timestamp).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(UserActivity::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserActivity {
    Table,
    Id,
    ProfileId,
    MediaId,
    ActivityType,
    ActivityData,
    Timestamp,
}
