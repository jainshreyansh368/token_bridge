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
                    .table(solana_bridge_transaction::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::TransactionHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::TransactionNonce)
                            .big_integer()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::TransactionStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::BridgeAmount)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::AdminFee)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::TransactionFee)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::Sender)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::Receiver)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::BridgedChain)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::RelatedClaimTransaction)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(solana_bridge_transaction::Column::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(solana_bridge_transaction::Entity)
                    .to_owned(),
            )
            .await
    }
}
