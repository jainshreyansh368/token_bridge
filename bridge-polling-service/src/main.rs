use chrono::Utc;
use figment::{
    providers::{Format, Toml},
    Figment,
};

use bridge_db_entity::{solana_bridge_non_parsed_transaction, solana_bridge_transaction};

use sea_orm::{
    entity::Set as EntitySet, prelude::Decimal, ActiveModelTrait, ActiveValue, ColumnTrait,
    ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    Statement,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use tokio::{task, time::sleep};
use tracing::{error, info, warn};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: config::Config = Figment::new().merge(Toml::file("App.toml")).extract()?;
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &config.rust_log);
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(
                format!("mandate-polling-service={}", &config.polling_service_log)
                    .parse()
                    .expect("Error parsing directive"),
            ),
        )
        .with_span_events(FmtSpan::FULL)
        .init();

    let db: DatabaseConnection = config::get_db_connection(&config).await?;
    let polling_batch_transactions: usize = match config.polling_batch_transactions {
        Some(v) => v,
        None => 1_000,
    };
    let polling_batch_sleep_millis = match config.polling_batch_sleep_millis {
        Some(v) => v,
        None => 100,
    };
    let polling_sleep_secs = match config.polling_sleep_secs {
        Some(v) => v,
        None => 10,
    };

    let client = reqwest::Client::builder()
        .build()
        .expect("Reqwest client failed to initialize!");

    // wait for other instance to shutdown before starting this loop
    sleep(Duration::from_secs(polling_sleep_secs)).await;

    // let datadog_client = datadog_apm::Client::new(datadog_apm::Config {
    //     env: Some("prod-nft".to_owned()),
    //     service: "prod-bridge-polling-service".to_owned(),
    //     host: config.datadog_host.to_owned(),
    //     port: config.datadog_port.to_owned(),
    //     ..Default::default()
    // });

    // let datadog_client = if config.enable_datadog {
    //     Some(&datadog_client)
    // } else {
    //     None
    // };

    loop {
        let latest_transaction = solana_bridge_transaction::Entity::find()
            .order_by_desc(solana_bridge_transaction::Column::CreatedAt)
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
            mut resp,
            mut last_processed_count,
            mut last_processed_signature,
            mut non_parsed_transactions,
        ) = get_transactions_and_account_info(
            &config,
            "".to_owned(),
            until,
            polling_batch_transactions,
            polling_batch_sleep_millis,
        )
        .await;

        insert_non_parsed_transactions(&db, &mut non_parsed_transactions).await;
        non_parsed_transactions.clear();

        // let mut bridge_data_accounts =
        //     update_accounts_transactions(&db, &config, &client, &resp).await;
        info!(
            "Last processed count, transaction: {:?}, {:?}",
            last_processed_count, last_processed_signature
        );

        while last_processed_count == polling_batch_transactions {
            if !last_processed_signature.is_empty() {
                last_processed_signature = "&before=".to_owned() + &last_processed_signature;
            }
            (
                resp,
                last_processed_count,
                last_processed_signature,
                non_parsed_transactions,
            ) = get_transactions_and_account_info(
                &config,
                last_processed_signature.to_owned(),
                "".to_owned(),
                polling_batch_transactions,
                polling_batch_sleep_millis,
            )
            .await;
            // insert_non_parsed_transactions(&db, &mut non_parsed_transactions).await;
            // non_parsed_transactions.clear();
            // info!(
            //     "Inner Last processed count, transaction: {:?}, {:?}",
            //     last_processed_count, last_processed_signature
            // );
            // let new_bridge_data_accounts =
            //     update_accounts_transactions(&db, &config, &client, &resp).await;
            // for new_account in new_bridge_data_accounts {
            //     if !bridge_data_accounts.contains(&new_account) {
            //         bridge_data_accounts.push(new_account);
            //     }
            // }
        }

        // if !bridge_data_accounts.contains(&config.bridge_state) {
        //     bridge_data_accounts.push(config.bridge_state.to_owned());
        // }

        // let bridge_data_account_objs = get_bridge_data_account_info(
        //     &config,
        //     bridge_data_accounts,
        //     polling_batch_sleep_millis,
        // )
        // .await;
        // update_bridge_accounts(&db, bridge_data_account_objs).await;

        // transaction::clear_old_encoded_transactions(&db).await;

        sleep(Duration::from_secs(polling_sleep_secs)).await;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    block_time: i64,
    error: bool,
    instruction_type: String,
    bridge_state: String,
    transaction_signature: String,
    amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockData {
    pub lock_transaction_sign: String,
    pub token_mint: String,
    pub senders_address: String,
    pub receivers_address: String,
    pub token_locked: u64,
}

async fn get_transactions_and_account_info(
    config: &config::Config,
    before: String,
    until: String,
    limit: usize,
    polling_batch_sleep_millis: u64,
) -> (HashMap<String, LockData>, usize, String, Vec<String>) {
    let solana_web_api_url = config.solana_web_api_node.to_owned()
        + "/get_transactions_and_account_info?limit="
        + &limit.to_string()
        + &before
        + &until;
    info!("solana url: {:?}", solana_web_api_url);

    let empty_tuple = (HashMap::new(), 0, before, vec![]);

    let resp = match reqwest::get(&solana_web_api_url).await {
        Ok(resp) => match resp.error_for_status() {
            Ok(resp) => match resp
                .json::<(HashMap<String, LockData>, usize, String, Vec<String>)>()
                .await
            {
                Ok(tuple_value) => tuple_value,
                Err(error) => {
                    warn!("Converting to json failed!: {:?}", error);
                    empty_tuple
                }
            },
            Err(error) => {
                warn!(
                    "get_transactions_and_account_info reqwest bad status {}: {:?}",
                    error.status().unwrap().to_string(),
                    error
                );
                empty_tuple
            }
        },
        Err(error) => {
            let status = if error.status().is_some() {
                error.status().unwrap().to_string()
            } else {
                "".to_owned()
            };
            warn!("bad response {}: {:?}", status, error);
            empty_tuple
        }
    };

    sleep(Duration::from_millis(polling_batch_sleep_millis)).await;
    resp
}

async fn insert_non_parsed_transactions(
    db: &DatabaseConnection,
    non_parsed_transactions: &Vec<String>,
) {
    let mut transactions: Vec<solana_bridge_non_parsed_transaction::ActiveModel> = vec![];
    for trx in non_parsed_transactions {
        let val = solana_bridge_non_parsed_transaction::ActiveModel {
            transaction_hash: ActiveValue::Set(trx.to_string()),
            attempt_timestamp: ActiveValue::Set(Utc::now().timestamp()),
        };
        transactions.push(val);
    }
    if transactions.is_empty() {
        return;
    }
    /*match non_parsed_transaction::Entity::insert_many(transactions)
        .on_conflict(
            OnConflict::column(non_parsed_transaction::Column::TransactionSignature)
                .do_nothing()
                .to_owned(),
        )
        .exec(db)
        .await
    {
        Ok(_) => {}
        Err(error) => {
            warn!(
                "Failed to insert non parsed transactions {:?}: {:?}",
                non_parsed_transactions, error
            );
        }
    }*/

    for transaction in transactions {
        match solana_bridge_non_parsed_transaction::Entity::insert(transaction)
            .exec(db)
            .await
        {
            Ok(_) => {}
            Err(db_error) => warn!(
                "Failed to insert non parsed transactions {:?}: {:?}",
                non_parsed_transactions, db_error
            ),
        }
    }
}
