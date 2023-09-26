use crate::{
    bridge::BridgeConfig,
    get_transactions_and_account_info,
    polling_service::db_actions::{insert_non_parsed_transactions, insert_transaction_details},
    routes::transaction,
};

use super::polling::{self, PollingConfig};
use std::error::Error;

use bridge_db_entity;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use sea_orm::{
    entity::Set as EntitySet, prelude::Decimal, ActiveModelTrait, ActiveValue, ColumnTrait,
    ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    Statement,
};
use solana_client::rpc_client::RpcClient;
use std::time::Duration;
use tokio::time::sleep;

pub async fn start_polling_service(polling_config: PollingConfig, bridge_config: BridgeConfig) {
    let db: DatabaseConnection = polling::get_db_connection(&polling_config).await.unwrap();
    let polling_batch_transactions: usize = match polling_config.polling_batch_transactions {
        Some(v) => v,
        None => 1_000,
    };
    let polling_batch_sleep_millis = match polling_config.polling_batch_sleep_millis {
        Some(v) => v,
        None => 100,
    };
    let polling_sleep_secs = match polling_config.polling_sleep_secs {
        Some(v) => v,
        None => 10,
    };

    let rpc_client = RpcClient::new(polling_config.on_chain_endpoint);

    loop {
        info!("Polling Transactions...");

        let latest_transaction = bridge_db_entity::solana_bridge_transaction::Entity::find()
            .order_by_desc(bridge_db_entity::solana_bridge_transaction::Column::CreatedAt)
            .one(&db)
            .await;

        let until = match latest_transaction {
            Ok(Some(trx)) => "&until=".to_owned() + &trx.transaction_hash,
            Ok(None) => "".to_owned(),
            Err(error) => {
                error!("Error: {:?}", error);
                sleep(Duration::from_millis(100)).await;
                continue;
            }
        };

        let (
            mut transactions,
            mut last_processed_count,
            mut last_processed_signature,
            mut non_parsed_transactions,
        ) = get_transactions_and_account_info(
            &rpc_client,
            &bridge_config,
            polling_config.polling_batch_transactions,
            None,
            Some(&until),
        )
        .await;

        insert_non_parsed_transactions(&db, &mut non_parsed_transactions).await;
        non_parsed_transactions.clear();

        insert_transaction_details(&db, &transactions).await;
        transactions.clear();

        while last_processed_count == polling_batch_transactions {
            if !last_processed_signature.is_empty() {
                last_processed_signature = "&before=".to_owned() + &last_processed_signature;
            }

            let (
                mut transactions,
                mut last_processed_count,
                mut last_processed_signature,
                mut non_parsed_transactions,
            ) = get_transactions_and_account_info(
                &rpc_client,
                &bridge_config,
                polling_config.polling_batch_transactions,
                Some(&last_processed_signature),
                None,
            )
            .await;

            insert_non_parsed_transactions(&db, &mut non_parsed_transactions).await;
            non_parsed_transactions.clear();

            insert_transaction_details(&db, &transactions).await;
            transactions.clear();

            sleep(Duration::from_secs(polling_sleep_secs)).await;
        }

        sleep(Duration::from_secs(polling_sleep_secs)).await;
    }
}
