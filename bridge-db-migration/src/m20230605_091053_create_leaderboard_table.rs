use bridge_db_entity::leaderboard;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(leaderboard::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(leaderboard::Column::Address)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(leaderboard::Column::Username)
                            .string()
                        )
                    .col(ColumnDef::new(leaderboard::Column::TotalPoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::StakePoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::UnstakePoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::SendPoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::ClaimPoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::BridgePoints).decimal())
                    .col(ColumnDef::new(leaderboard::Column::FaucetPoints).decimal())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(leaderboard::Entity).to_owned())
            .await
    }
}
