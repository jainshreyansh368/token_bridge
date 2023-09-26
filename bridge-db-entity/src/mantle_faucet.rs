use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "faucets", schema_name = "public")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub transaction_hash: String,
    pub address: String,
    pub token: String,
    pub last_received: String,
    pub transaction_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

