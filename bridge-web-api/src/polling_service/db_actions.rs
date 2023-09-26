use bridge_db_entity::{
    solana_bridge_non_parsed_transaction, solana_bridge_transaction, solana_claim_transaction, configurations::Model,
};
use chrono::Utc;
use log::error;
use sea_orm::{
    prelude::Decimal, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use tracing::warn;

use crate::rpc_wrapper::Transaction;

pub async fn insert_non_parsed_transactions(
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

pub async fn insert_transaction_details(db: &DatabaseConnection, transactions: &Vec<Transaction>) {
    let bridge_transactions: Vec<&Transaction> = transactions
        .into_iter()
        .filter(|trx| trx.instruction_type == "bridge_token")
        .collect();

    let claim_transactions: Vec<&Transaction> = transactions
        .into_iter()
        .filter(|trx| trx.instruction_type == "claim_token")
        .collect();

    if !bridge_transactions.is_empty() {
        insert_bridge_transactions(db, bridge_transactions).await;
    }

    if !claim_transactions.is_empty() {
        insert_claim_transactions(db, claim_transactions).await;
    }
}

async fn insert_bridge_transactions(
    db: &DatabaseConnection,
    bridge_transactions: Vec<&Transaction>,
) {
    info!("Insert Bridge Transactions Started");
    let mut transactions: Vec<solana_bridge_transaction::ActiveModel> = Vec::new();

    for trx in bridge_transactions {
        let latest_transaction = bridge_db_entity::solana_bridge_transaction::Entity::find()
            .filter(
                bridge_db_entity::solana_bridge_transaction::Column::TransactionHash
                    .contains(&trx.transaction_signature),
            )
            .one(db)
            .await;

        match latest_transaction {
            Ok(Some(_)) => continue,
            Ok(None) => (),
            Err(db_err) => {
                error!("Unable to get data from db: {:?}", db_err);
                continue;
            }
        }

        let entry = bridge_db_entity::solana_bridge_transaction::ActiveModel {
            transaction_hash: ActiveValue::Set(trx.transaction_signature.to_owned()),
            transaction_status: if !trx.error {
                ActiveValue::Set("Success".to_string())
            } else {
                ActiveValue::Set("Failed".to_string())
            },
            bridge_amount: ActiveValue::Set(Decimal::from(trx.amount)),
            admin_fee: ActiveValue::Set(Decimal::from(0)),
            transaction_fee: ActiveValue::Set(Decimal::from(0)),
            sender: ActiveValue::Set(trx.sender.to_owned()),
            receiver: ActiveValue::Set(trx.reciever.to_owned()),
            bridged_chain: ActiveValue::Set("Mantle".to_string()),
            related_claim_transaction: ActiveValue::Set(None),
            transaction_nonce: ActiveValue::default(),
            created_at: ActiveValue::Set(trx.block_time),
            updated_at: ActiveValue::Set(trx.block_time),
        };

        transactions.push(entry);
    }

    for tx in transactions {
        match bridge_db_entity::solana_bridge_transaction::Entity::insert(tx)
            .exec(db)
            .await
        {
            Ok(_) => (),
            Err(db_err) => warn!("Failed to insert solana bridge transaction: {:?}", db_err),
        }
    }
    info!("Insert Bridge Transactions Completed");
}

async fn insert_claim_transactions(db: &DatabaseConnection, claim_transactions: Vec<&Transaction>) {
    info!("Caim Transactions Started");
    let mut transactions: Vec<solana_claim_transaction::ActiveModel> = Vec::new();

    for trx in claim_transactions {
        let latest_transaction = bridge_db_entity::solana_claim_transaction::Entity::find()
            .filter(
                bridge_db_entity::solana_claim_transaction::Column::TransactionHash
                    .contains(&trx.transaction_signature),
            )
            .one(db)
            .await;

        match latest_transaction {
            Ok(Some(_)) => continue,
            Ok(None) => (),
            Err(db_err) => {
                error!("Unable to get data from db: {:?}", db_err);
                continue;
            }
        }

        let entry = bridge_db_entity::solana_claim_transaction::ActiveModel {
            transaction_hash: ActiveValue::Set(trx.transaction_signature.to_owned()),
            transaction_status: if !trx.error {
                ActiveValue::Set("Success".to_string())
            } else {
                ActiveValue::Set("Failed".to_string())
            },
            claim_amount: ActiveValue::Set(Decimal::from(trx.amount)),
            receiver: ActiveValue::Set(trx.reciever.to_owned()),
            related_transaction_hash: ActiveValue::Set(None),
            bridged_chain: ActiveValue::Set("Mantle".to_string()),
            created_at: ActiveValue::Set(trx.block_time),
            updated_at: ActiveValue::Set(trx.block_time),
        };

        transactions.push(entry);
    }

    for tx in transactions {
        match bridge_db_entity::solana_claim_transaction::Entity::insert(tx)
            .exec(db)
            .await
        {
            Ok(_) => (),
            Err(db_err) => warn!("Failed to insert solana claim transaction: {:?}", db_err),
        }
    }
    info!("Insert Claim Transactions Completed");
}

// async fn insert_mantle_bridge_transaction(
//     claim_txn: Model
// ) {
//     let latest_transaction = bridge_db_entity::mantle_bridge_transactions::Entity::find()
//     .filter(
//         bridge_db_entity::mantle_bridge_transactions::Column::TransactionHash
//             .contains(bridge_db_entity::solana_claim_transaction::Column:::rela),
//     )
//     .one(db)
//     .await;

//     match latest_transaction {
//         Ok(Some(_)) => continue,
//         Ok(None) => (),
//         Err(db_err) => {
//             error!("Unable to get data from db: {:?}", db_err);
//             continue;
//         }
//     }


// }
