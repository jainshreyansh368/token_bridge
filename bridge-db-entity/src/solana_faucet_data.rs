use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "solana_faucet_data", schema_name = "public")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub user: String,
    pub token: String,
    pub last_claimed_timestamp: i64,
    pub created_at: i64,
    pub updated_at: i64
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// #[derive(EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
// pub enum TokenType {
//     #[sea_orm(string_value = "gari")]
//     Gari,
//     #[sea_orm(string_value = "bit")]
//     Bit,
// }