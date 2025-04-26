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
                    .table(Peer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Peer::Id).uuid().not_null().primary_key())
                    .col(string(Peer::PeerId).not_null().unique_key()) // Unique identifier for the peer
                    .col(string(Peer::IpAddress).not_null()) // IP address of the peer
                    .col(integer(Peer::Port).not_null()) // Port number the peer is listening on
                    .col(timestamp(Peer::LastSeen).default(Expr::current_timestamp())) // Last time we heard from this peer
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Peer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Peer {
    // Added for P2P
    Table,
    Id,
    PeerId,
    IpAddress,
    Port,
    LastSeen,
}
