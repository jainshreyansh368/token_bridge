use bridge_db_entity::solana_faucet_data;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(solana_faucet_data::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(solana_faucet_data::Column::User)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(solana_faucet_data::Column::Token).string().not_null())
                    .col(ColumnDef::new(solana_faucet_data::Column::LastClaimedTimestamp).big_integer().not_null())
                    .col(ColumnDef::new(solana_faucet_data::Column::CreatedAt).big_integer().not_null())
                    .col(ColumnDef::new(solana_faucet_data::Column::UpdatedAt).big_integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(solana_faucet_data::Entity).to_owned())
            .await
    }
}
