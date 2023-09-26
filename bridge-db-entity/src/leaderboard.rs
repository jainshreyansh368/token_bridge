use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "leaderboard", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub address: String,
    pub username: String,
    pub total_points: Decimal,
    pub stake_points: Decimal,
    pub unstake_points: Decimal,
    pub send_points: Decimal,
    pub claim_points: Decimal,
    pub bridge_points: Decimal,
    pub faucet_points: Decimal,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
