use bridge_db_entity::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(solana_bridge_non_parsed_transaction::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(
                            solana_bridge_non_parsed_transaction::Column::TransactionHash,
                        )
                        .string()
                        .not_null()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(
                            solana_bridge_non_parsed_transaction::Column::AttemptTimestamp,
                        )
                        .big_integer(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(solana_bridge_non_parsed_transaction::Entity)
                    .to_owned(),
            )
            .await
    }
}
