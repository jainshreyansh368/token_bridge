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
                    .table(mantle_bridge_transactions::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::TransactionHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::TransactionStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::BridgeAmount)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::AdminFee)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::TransactionFee)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::Sender)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::Receiver)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::RelatedClaimTransaction)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_bridge_transactions::Column::UpdatedAt)
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
                    .table(mantle_bridge_transactions::Entity)
                    .to_owned(),
            )
            .await
    }
}
