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
                    .table(Library::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Library::Id).uuid().not_null().primary_key())
                    .col(string(Library::Name).not_null())
                    .col(string(Library::Path).not_null()) // Path to the media library on disk
                    .col(timestamp(Library::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Library::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Library::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Library {
    Table,
    Id,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
}
