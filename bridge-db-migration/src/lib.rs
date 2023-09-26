pub use sea_orm_migration::prelude::*;

mod m20230522_093612_create_solana_bridge_non_parsed_transactions_table;
mod m20230525_182113_create_solana_bridge_transactions_table;
mod m20230525_182126_create_solana_claim_transactions_table;
mod m20230530_111142_create_mantle_claim_transactions_table;
mod m20230530_115914_create_mantle_bridge_transactions_table;
mod m20230605_082834_update_mantle_bridge_tx_table;
mod m20230605_085129_update_solana_bridge_tx_table;
mod m20230605_085702_update_solana_claim_tx_table;
mod m20230605_085938_update_mantle_claim_tx_table;
mod m20230605_091047_create_configurations_table;
mod m20230605_091053_create_leaderboard_table;
mod m20230608_110551_solana_faucet_data;
mod m20230616_053720_transactions;
mod m20230703_095606_faucet;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230522_093612_create_solana_bridge_non_parsed_transactions_table::Migration),
            Box::new(m20230525_182113_create_solana_bridge_transactions_table::Migration),
            Box::new(m20230525_182126_create_solana_claim_transactions_table::Migration),
            Box::new(m20230530_111142_create_mantle_claim_transactions_table::Migration),
            Box::new(m20230530_115914_create_mantle_bridge_transactions_table::Migration),
            Box::new(m20230605_082834_update_mantle_bridge_tx_table::Migration),
            Box::new(m20230605_085129_update_solana_bridge_tx_table::Migration),
            Box::new(m20230605_085702_update_solana_claim_tx_table::Migration),
            Box::new(m20230605_085938_update_mantle_claim_tx_table::Migration),
            Box::new(m20230605_091047_create_configurations_table::Migration),
            Box::new(m20230605_091053_create_leaderboard_table::Migration),
            Box::new(m20230608_110551_solana_faucet_data::Migration),
            Box::new(m20230616_053720_transactions::Migration),
            Box::new(m20230703_095606_faucet::Migration),
        ]
    }
}
