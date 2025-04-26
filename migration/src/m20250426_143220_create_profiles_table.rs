use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Profile::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Profile::ParentId).uuid().null())
                    .col(string(Profile::Email).not_null().unique_key())
                    .col(ColumnDef::new(Profile::Password).string().null())
                    .col(ColumnDef::new(Profile::Phone).string().null())
                    .col(string(Profile::Name).not_null())
                    .col(
                        ColumnDef::new(Profile::Avatar)
                            .string()
                            .default("https://example.com/default-avatar.png"),
                    )
                    .col(ColumnDef::new(Profile::Pin).string().default("1234"))
                    .col(ColumnDef::new(Profile::UsePin).boolean().default(false))
                    .col(timestamp(Profile::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Profile::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-profile-parent_id")
                            .from(Profile::Table, Profile::ParentId)
                            .to(Profile::Table, Profile::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Profile {
    Table,
    Id,
    ParentId,
    Email,
    Phone,
    Password,
    Name,
    Avatar,
    UsePin,
    Pin,
    CreatedAt,
    UpdatedAt,
}
