use bridge_db_entity::{mantle_claim_transactions, solana_bridge_transaction};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_mantle_claim_solana_bridge";
        let foreign_key = TableForeignKey::new()
            .name(name)
            .from_tbl(mantle_claim_transactions::Entity)
            .from_col(mantle_claim_transactions::Column::RelatedTransactionHash)
            .to_tbl(solana_bridge_transaction::Entity)
            .to_col(solana_bridge_transaction::Column::TransactionHash)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(mantle_claim_transactions::Entity)
                    .add_foreign_key(&foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_mantle_claim_solana_bridge";
        manager
            .alter_table(
                Table::alter()
                    .table(mantle_claim_transactions::Entity)
                    .drop_foreign_key(Alias::new(name))
                    .to_owned(),
            )
            .await
    }
}
