use bridge_db_entity::{mantle_faucet};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(mantle_faucet::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(mantle_faucet::Column::TransactionHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(mantle_faucet::Column::Address).string().not_null())
                    .col(ColumnDef::new(mantle_faucet::Column::Token).string().not_null())
                    .col(ColumnDef::new(mantle_faucet::Column::LastReceived).string().not_null())
                    .col(ColumnDef::new(mantle_faucet::Column::TransactionType).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
