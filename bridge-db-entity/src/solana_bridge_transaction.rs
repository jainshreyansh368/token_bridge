use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "solana_bridge_transactions", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub transaction_hash: String,
    pub transaction_status: String,
    pub bridge_amount: Decimal,
    pub admin_fee: Decimal,
    pub transaction_fee: Decimal,
    pub sender: String,
    pub receiver: String,
    pub bridged_chain: String,
    pub related_claim_transaction: Option<String>,
    #[sea_orm(auto_increment = true)]
    pub transaction_nonce: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
