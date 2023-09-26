use bridge_db_entity::{mantle_claim_transactions, solana_bridge_transaction};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_solana_bridge_mantle_claim";
        let foreign_key = TableForeignKey::new()
            .name(name)
            .from_tbl(solana_bridge_transaction::Entity)
            .from_col(solana_bridge_transaction::Column::RelatedClaimTransaction)
            .to_tbl(mantle_claim_transactions::Entity)
            .to_col(mantle_claim_transactions::Column::TransactionHash)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(solana_bridge_transaction::Entity)
                    .add_foreign_key(&foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_solana_bridge_mantle_claim";
        manager
            .alter_table(
                Table::alter()
                    .table(solana_bridge_transaction::Entity)
                    .drop_foreign_key(Alias::new(name))
                    .to_owned(),
            )
            .await
    }
}
