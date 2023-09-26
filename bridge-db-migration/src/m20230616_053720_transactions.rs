use bridge_db_entity::transactions;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(transactions::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(transactions::Column::TransactionHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(transactions::Column::TransactionType).string().not_null())
                    .col(ColumnDef::new(transactions::Column::Address).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(transactions::Entity).to_owned())
            .await
    }
}

