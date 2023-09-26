use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "solana_claim_transactions", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub transaction_hash: String,
    pub transaction_status: String,
    pub claim_amount: Decimal,
    pub receiver: String,
    pub bridged_chain: String,
    pub related_transaction_hash: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
