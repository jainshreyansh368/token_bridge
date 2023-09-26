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
                    .table(bridge_db_entity::mantle_claim_transactions::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::TransactionHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::TransactionStatus)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::UniqueSeq)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::ClaimAmount)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::Receiver)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::RelatedTransactionHash)
                            .string(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(mantle_claim_transactions::Column::UpdatedAt)
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
                    .table(mantle_claim_transactions::Entity)
                    .to_owned(),
            )
            .await
    }
}
