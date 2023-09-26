use bridge_db_entity::{mantle_bridge_transactions, solana_claim_transaction};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_mantle_bridge_solana_claim";
        let foreign_key = TableForeignKey::new()
            .name(name)
            .from_tbl(mantle_bridge_transactions::Entity)
            .from_col(mantle_bridge_transactions::Column::RelatedClaimTransaction)
            .to_tbl(solana_claim_transaction::Entity)
            .to_col(solana_claim_transaction::Column::TransactionHash)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(mantle_bridge_transactions::Entity)
                    .add_foreign_key(&foreign_key)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let name = "fk_mantle_bridge_solana_claim";
        manager
            .alter_table(
                Table::alter()
                    .table(mantle_bridge_transactions::Entity)
                    .drop_foreign_key(Alias::new(name))
                    .to_owned(),
            )
            .await
    }
}
