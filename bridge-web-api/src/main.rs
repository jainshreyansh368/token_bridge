mod bridge;
mod instruction_data;
mod polling_service;
mod routes;
mod rpc_wrapper;

use bridge::{Db, BridgeConfig};
use base64::{engine::general_purpose, Engine as _};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use log::warn;
use reqwest::header::{AUTHORIZATION, HeaderValue};
use reqwest::Client;
use std::{time::{SystemTime, UNIX_EPOCH}, thread::current};
use rocket::{
    serde::json::Json, 
    Config, 
    State, 
    form::{Error, error},
    http::Header,
    tokio::time::{Duration, timeout},
    tokio::runtime::{Builder, Runtime},
    tokio::fs,
};
use sea_orm::{entity::Set as EntitySet, EntityTrait, ColumnTrait, QueryFilter, DatabaseConnection, Statement, ConnectionTrait, DbBackend, IntoActiveModel, ActiveModelTrait, ActiveValue};
use solana_client::{
    rpc_client::RpcClient, 
    client_error::ClientError
};
use solana_sdk::{
    message::Message, 
    pubkey::Pubkey, 
    transaction::Transaction, 
    signer::keypair,
    signature::{Keypair, Signature},
};
use std::collections::{HashMap, HashSet};
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
use routes::transaction::{*, self};
use sea_orm_rocket::{Connection, Database};
use bridge_db_entity::{solana_faucet_data::{
    Column as FaucetDataColumn, Entity as FaucetData,
    Model as FaucetDataModel, self,
}, configurations::Model
};

pub const DB_BACKEND: DbBackend = DbBackend::Postgres;

#[macro_use]
extern crate rocket;
extern crate tracing;

async fn get_transactions_and_account_info(
    rpc_client: &RpcClient,
    bridge_config: &bridge::BridgeConfig,
    limit: Option<usize>,
    before: Option<&str>,
    until: Option<&str>,
) -> (Vec<rpc_wrapper::Transaction>, usize, String, Vec<String>) {
    let signatures = rpc_wrapper::get_signatures(
        rpc_client,
        &bridge_config.bridge_program_address,
        limit,
        before,
        until,
    );
    let mut total_signatures_processed: usize = 0;
    let last_signature: String;
    let mut non_parsed_transactions_set: HashSet<String> = HashSet::new();
    let mut non_parsed_transactions: Vec<String> = vec![];
    let user_transactions: Vec<rpc_wrapper::Transaction> = match signatures {
        Some(signatures) => {
            total_signatures_processed = signatures.len();
            last_signature = signatures.last().unwrap().to_string();
            rpc_wrapper::get_user_data_account_with_transactions(
                rpc_client,
                &bridge_config.bridge_account_address,
                &bridge_config.fee_payer_address,
                signatures,
                &mut non_parsed_transactions_set,
                &mut non_parsed_transactions,
                true,
            )
            .await
        }
        None => {
            last_signature = String::from("");
            Vec::new()
        }
    };
    // rpc_wrapper::get_staking_user_account_info(rpc_client.inner(), &mut user_transactions).await;
    (
        user_transactions,
        total_signatures_processed,
        last_signature,
        non_parsed_transactions,
    )
}

#[get("/")]
async fn index() -> String {
    String::from("Hello World!")
}

#[get(
    "/get_encoded_transaction?<api_header>&<user>&<instruction_type>&<receiver_address>&<user_token_account>&<lock_transaction_hash>&<amount>&<mantle_gas_fee>&<transaction_nonce>"
)]
pub async fn get_bridge_transaction(
    rpc_client: &State<RpcClient>,
    bridge_config: &State<bridge::BridgeConfig>,
    api_header: Option<String>,
    user: String,
    instruction_type: String,
    receiver_address: String,
    user_token_account: Option<String>,
    lock_transaction_hash: Option<String>,
    amount: u64,
    mantle_gas_fee: Option<u64>,
    transaction_nonce: Option<u64>,
) -> Json<Result<String, String>> {

    // if instruction_type.eq("unlock_token") {
    //     make_request_with_oauth()
        
    // }

    let encoded_transaction = encode(
        rpc_client,
        bridge_config,
        &instruction_type,
        bridge_config.gari_faucet_account.clone(),
        user,
        receiver_address,
        user_token_account,
        lock_transaction_hash,
        amount,
        mantle_gas_fee,
        transaction_nonce,
    ).await;

    if instruction_type.eq("lock_token") {
        
        let partially_signed_tx = sign(
            bridge_config.admin_private_key.clone(), 
            encoded_transaction.unwrap()
        );

        Json(Ok(partially_signed_tx))
        // let signed_tx = sign(
        //     "4aZHEwS7tfM5mzfb8JGUKXVTSyQ1ejjcuArjQku9dmvK436UMzeeyWNWcNdFAv1cRphiyXbJQJheEzoTCkiMYqZz".to_string(), 
        //     partially_signed_tx
        // );
        // send(rpc_client, bridge_config, signed_tx.to_owned())
    }

    else if instruction_type.eq("unlock_token") {
        let signed_tx = sign(
            bridge_config.admin_private_key.clone(), 
            encoded_transaction.unwrap()
        );
        send(rpc_client, bridge_config, signed_tx.to_owned())
    }
    else {
        Json(Err("Invalid instruction type".to_string()))
    }
    

}



#[get(
    "/gari_faucet?<user>"
)]
pub async fn faucet_gari(
    conn: Connection<'_, Db>,
    rpc_client: &State<RpcClient>,
    bridge_config: &State<bridge::BridgeConfig>,
    user: String
) -> Json<Result<(String, String), String>>{

    //faucet data check using find function on db 
    let db = conn.into_inner();
    let transactions = FaucetData::find()
        .filter(FaucetDataColumn::User.eq(user.clone()))
        .all(db)
        .await;

    let mut last_airdrop_time : i64 = 0;
    let mut txn_model: Option<FaucetDataModel> = None;
    match transactions{
        Ok(data) => {
            if data.is_empty(){
                // txn_model = FaucetDataModel::new();
            }
            else{
                txn_model = Some(data[0].clone());
                println!("transaction model: {:?}", txn_model.clone().unwrap());
                last_airdrop_time = data[0].last_claimed_timestamp;
            }  
        }
        Err(error) => {
            return Json(Err("error fetching faucet data".to_string()));
        }
    } 
    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let current_time = since_the_epoch.as_secs() as i64;

    println!("current time : {:?}" , current_time);
    println!("last airdrop : {:?}" , last_airdrop_time);

    // change to 24 hours, now 5 hours
    if last_airdrop_time != 0 {
        let faucet_enable_time = last_airdrop_time.checked_add(18000 as i64).unwrap();
        
        println!("faucet limit : {:?}" , faucet_enable_time);

    //     if current_time < faucet_enable_time {
    //         return Json(Err("faucet limit exceeded".to_string()));
    //     }
    }

        // let sol_faucet_txn = sol_faucet(
        //     rpc_client, 
        //     user.clone(), 
        //     1000000000).unwrap().to_string();
        let sol_faucet_txn =  "5RpbGksRsjB69qmwKN74GqS2cPWBYp38aWTdVskMMgPfg7pW6SwEmZFXEi77CUzsrK8DfBGXYV3xoJYkXkrhsuY8".to_string();

        let gari_faucet_txn = gari_faucet(rpc_client, bridge_config, user.clone(), 1000000000).await.unwrap().to_string();

        if txn_model.is_some() {
            println!("updating faucet record");
            let mut active_transaction = txn_model.unwrap().into_active_model();
            active_transaction.last_claimed_timestamp = EntitySet(current_time);
            active_transaction.updated_at = EntitySet(current_time);
            match active_transaction.update(db).await {
                Ok(_) => {}
                Err(error) => warn!(
                    "Could not update transaction {}: {:?}",
                    last_airdrop_time, error
                ),
            }
        }
        else {

            let faucet_transaction = bridge_db_entity::solana_faucet_data::ActiveModel {
                user: ActiveValue::Set(user),
                token: ActiveValue::Set("gari".to_string()),
                last_claimed_timestamp: ActiveValue::Set(current_time),
                created_at: ActiveValue::Set(current_time),
                updated_at: ActiveValue::Set(current_time)
            };

            faucet_transaction.insert(db).await.unwrap();
            // faucet_transaction.insert(&entry);
        }
        
        
        Json(Ok((sol_faucet_txn, gari_faucet_txn)))

}

#[get(
    "/bridge_estimation?<amount>"
)]
pub async fn bridge_estimation(
    bridge_config: &State<bridge::BridgeConfig>,
    amount: u64
) -> Json<Result<u64, String>>{

    let final_amount = amount
        .checked_sub(bridge_config.bridge_platform_fee)
        .ok_or(warn!("overflow error")).unwrap()
        .checked_sub(bridge_config.mantle_gas_fee)
        .ok_or(warn!("overflow error")).unwrap();

    Json(Ok(final_amount))
}

// #[get(
//     "/claim_request?<api_header>&<user>&<instruction_type>&<receiver_address>&<user_token_account>&<lock_transaction_hash>&<amount>&<mantle_gas_fee>&<transaction_nonce>"
// )]
// // Function to make a GET request with the OAuth API key
// async fn claim_request() -> Result() {
//     let api_key = "BRIDGE_OAUTH_API_KEY";
//     let authorization_header = HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap();
//     let client = Client::builder()
//         .default_headers(vec![Header::new(AUTHORIZATION, authorization_header)])
//         .build()?;

//     let response = client
//         .get("https://api.example.com/endpoint")
//         .send()
//         .await?;

//     // Process the response as needed
//     // For example, you can print the response body
//     let body = response.text().await?;
//     println!("Response body: {}", body);

//     Ok(())
// }



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let bridge_config = Config::figment().extract::<bridge::BridgeConfig>().unwrap();
    let bridge_config_polling = bridge_config.clone();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &bridge_config.rust_log);
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive(
                format!("bridge_web_api={}", &bridge_config.bridge_web_api_log)
                    .parse()
                    .expect("Error parsing directive"),
            ),
        )
        .with_span_events(FmtSpan::FULL)
        .init();

    tokio::spawn(async {
        let config: polling_service::polling::PollingConfig = Figment::new()
            .merge(Toml::file("App.toml"))
            .extract()
            .unwrap();
        polling_service::service::start_polling_service(config, bridge_config_polling).await
    });

    let rpc_client = RpcClient::new(&bridge_config.on_chain_endpoint);
    rocket::build()
        .manage(rpc_client)
        .manage(bridge_config)
        .attach(routes::mount())
        .attach(Db::init())
        .mount("/", routes![
            index, 
            get_bridge_transaction,
            faucet_gari,
            bridge_estimation
            ])
        .ignite()
        .await?
        .launch()
        .await?;
    Ok(())
}
