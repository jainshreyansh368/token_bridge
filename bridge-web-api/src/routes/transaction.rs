
use crate::{bridge, instruction_data, rpc_wrapper};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use rocket::{get, serde::json::Json, State, http::private::Connection};
use sea_orm::ConnectOptions;
use solana_client::{
    rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig,
    rpc_config::RpcSimulateTransactionConfig,
    client_error::ClientError,
};
use solana_program::instruction::Instruction;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    transaction::{Transaction, TransactionError},
};
use solana_transaction_status::parse_instruction::parse;
use spl_token::instruction::transfer;


pub async fn encode(
    rpc_client: &State<RpcClient>,
    bridge_config: &State<bridge::BridgeConfig>,
    instruction_type: &String,
    bridge_state: String,
    admin: String,
    receiver_address: String,
    user_token_account: Option<String>,
    lock_transaction_hash: Option<String>,
    amount: u64,
    mantle_gas_fee: Option<u64>,
    transaction_nonce: Option<u64>,
) -> Result<String, String> {
    let program_id = bridge_config
        .bridge_program_address
        .parse::<Pubkey>()
        .unwrap();

    println!("instruction type: {}",instruction_type);
    let bridge_instruction = if instruction_type.eq("lock_token") {
        println!("instruction bridge: ");

        if mantle_gas_fee.is_some() {
            bridge::Instruction::LOCK
        } else {
            return Err("Wrong instruction type".to_string());
        }
    }
     else if instruction_type.eq("unlock_token") {
        println!("instruction claim: ");

        if transaction_nonce.is_some() {
            bridge::Instruction::UNLOCK
        } else {
            return Err("Wrong instruction type".to_string());
        }
    }
    else {
        return Err("wrong instruction type".to_string());
    };
    // else {
    //     return Json(Err("Wrong instruction type".to_string()));
    // };

    // // if not in db, then initialize
    // let user_mandate_state = if user_mandate_state.is_none() {
    //     // expensive calculation
    //     let user = match user.parse::<Pubkey>() {
    //         Ok(pubkey) => pubkey,
    //         Err(error) => {
    //             warn!("Invalid user pubkey {}: {}", user, error);
    //             return Json(Err("Invalid Public key sent".to_string()));
    //         }
    //     };
    //     let (account, _bump_seed) = Pubkey::find_program_address(
    //         &["mandate_data".as_ref(), &user.to_bytes()],
    //         &bridge_config
    //             .mandate_program_address
    //             .parse::<Pubkey>()
    //             .unwrap(),
    //     );
    //     account
    // } else {
    //     user_mandate_state.unwrap().parse::<Pubkey>().unwrap()
    // };

    // let token_acc_owner = if instruction_type.eq("unlock_token"){
    //     receiver_address.clone()
    // } else {
    //     user.clone()
    // };

    // let (user_token_account, associated_instruction) = if user_token_account.is_none() {
    //     rpc_wrapper::get_associated_account(
    //         &rpc_client,
    //         &token_acc_owner,
    //         &bridge_config.fee_payer_address,
    //         &bridge_config.bridge_account_token_mint,
    //     ).await
    // } else {
    //     (user_token_account.unwrap().parse::<Pubkey>().unwrap(), None)
    // };

    let main_instruction = match bridge_instruction {
        bridge::Instruction::LOCK => instruction_data::lock_tokens(
            program_id,
            &instruction_data::InstructionLockToken {
                instruction_type: bridge::Instruction::LOCK,
                amount,
                mantle_gas_fee: mantle_gas_fee.unwrap(),
                receiver_address,
            },
            bridge_config.bridge_admin_address.parse::<Pubkey>().unwrap(),
            bridge_config.bridge_account_token_mint.parse::<Pubkey>().unwrap(),
            bridge_state.parse::<Pubkey>().unwrap(),
            bridge_config.bridge_holding_wallet.parse::<Pubkey>().unwrap(),
            bridge_config.admin_token_account.parse::<Pubkey>().unwrap(),
            bridge_config.gari_treassury_wallet.parse::<Pubkey>().unwrap(),

        ),
        _ => {
            let (claimed_transaction_state, _nonce) = Pubkey::find_program_address(
                &[transaction_nonce.unwrap().to_string().as_ref()], 
                &program_id
            );

            instruction_data::claim_locked_tokens(
                program_id,
                &instruction_data::InstructionUnlockToken {
                    instruction_type: bridge::Instruction::UNLOCK,
                    lock_transaction_hash: lock_transaction_hash.unwrap(),
                    amount,
                    transaction_nonce: transaction_nonce.unwrap(),
                },
                bridge_config.bridge_admin_address.parse::<Pubkey>().unwrap(),
                bridge_config.bridge_account_token_mint.parse::<Pubkey>().unwrap(),
                bridge_state.parse::<Pubkey>().unwrap(),
                claimed_transaction_state,
                bridge_config.bridge_holding_wallet.parse::<Pubkey>().unwrap(),
                bridge_config.admin_token_account.parse::<Pubkey>().unwrap(),
            )
        }

    };

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    // let instructions = match associated_instruction {
    //     Some(associated_instruction) => {
    //         vec![associated_instruction, main_instruction]
    //     }
    //     None => {
    //         vec![main_instruction]
    //     }
    // };

    let instructions = vec![main_instruction];

    let fee_payer = bridge_config.fee_payer_address.parse::<Pubkey>().unwrap();
    let message = Message::new_with_blockhash(&instructions, Some(&fee_payer), &blockhash);
    let trx: Vec<u8> = bincode::serialize(&Transaction::new_unsigned(message)).unwrap();

    Ok(general_purpose::STANDARD.encode(trx))
}


pub fn send(
    rpc_client: &State<RpcClient>,
    bridge_config: &State<bridge::BridgeConfig>,
    encoded_transaction: String,
) -> Json<Result<String, String>> {
    let mut transaction = bincode::deserialize::<Transaction>(
        general_purpose::STANDARD
            .decode(encoded_transaction)
            .unwrap()
            .as_slice(),
    )
    .unwrap();

    let keypair = Keypair::from_base58_string(&bridge_config.fee_payer_private_key);
    transaction.partial_sign(&[&keypair], transaction.message.recent_blockhash);

    let simulation_result = if bridge_config.send_transaction_simulate {
        let rpc_simulate_transaction_config = RpcSimulateTransactionConfig {
            sig_verify: true,
            commitment: Some(CommitmentConfig::confirmed()),
            ..RpcSimulateTransactionConfig::default()
        };

        match rpc_client
            .simulate_transaction_with_config(&transaction, rpc_simulate_transaction_config)
        {
            Ok(result) => match result.value.err {
                Some(TransactionError::InstructionError(_, _)) => {
                    let mut log = String::new();
                    for l in result.value.logs.unwrap() {
                        log.push_str(&l);
                        log.push_str("  ");
                    }

                    Some(Json(Err(format!("{}", log))))
                }
                _ => None,
            },
            Err(error) => Some(Json(Err(error.to_string()))),
        }
    } else {
        None
    };

    if simulation_result.is_some() {
        return simulation_result.unwrap();
    }

    let rpc_send_transaction_config = RpcSendTransactionConfig {
        skip_preflight: false,
        preflight_commitment: Some(CommitmentLevel::Confirmed),
        ..RpcSendTransactionConfig::default()
    };

    match rpc_client.send_transaction_with_config(&transaction, rpc_send_transaction_config) {
        Ok(result) => {
            let result = result.to_string();
            if result.eq(instruction_data::SOL_ADDRESS) {
                Json(Err("Transaction dropped!".to_owned()))
            } else {
                Json(Ok(result))
            }
        }
        Err(error) => Json(Err(error.to_string())),
    }
}

pub fn sign(user_private_key: String, encoded_transaction: String) -> String {
    let mut transaction = bincode::deserialize::<Transaction>(
        general_purpose::STANDARD
            .decode(encoded_transaction)
            .unwrap()
            .as_slice(),
    )
    .unwrap();

    let keypair = Keypair::from_base58_string(&user_private_key);
    transaction.partial_sign(&[&keypair], transaction.message.recent_blockhash);
    let trx: Vec<u8> = bincode::serialize(&transaction).unwrap();
    general_purpose::STANDARD.encode(trx)
}

pub async fn gari_faucet(
    rpc_client: &State<RpcClient>,
    bridge_config: &State<bridge::BridgeConfig>,
    user: String,
    amount: u64
) -> Result<Signature, String> {


    // let mut opt = ConnectOptions::new("postgres://custom_bridge:custom_bridge@localhost:5432/custom_bridge".to_string());

    let (user_token_account, associated_instruction) = 
        rpc_wrapper::get_associated_account(
            &rpc_client,
            &user,
            &bridge_config.faucet_admin_address,
            &bridge_config.gari_faucet_token_mint,
    ).await;

    let faucet_instruction = transfer(
        &spl_token::id(),
        &bridge_config.gari_faucet_account.parse::<Pubkey>().unwrap(),
        &user_token_account,
        &bridge_config.faucet_admin_address.parse::<Pubkey>().unwrap(),
        &[],
        amount
    ).unwrap();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let instructions = match associated_instruction {
        Some(associated_instruction) => {
            vec![associated_instruction, faucet_instruction]
        }
        None => {
            vec![faucet_instruction]
        }
    };

    let mut tx = Transaction::new_with_payer(
        &instructions, 
        Some(&bridge_config.faucet_admin_address.parse::<Pubkey>().unwrap())
    );
    let keypair = Keypair::from_base58_string(&bridge_config.faucet_admin_private_key);
    tx.sign(&[&keypair], blockhash);
    Ok(rpc_client.send_and_confirm_transaction(&tx).unwrap())
}
  

pub fn sol_faucet(
    rpc_client: &State<RpcClient>,
    user: String,
    amount: u64,

) -> Result<Signature, String> {

    let result = rpc_client.request_airdrop(
        &user.parse::<Pubkey>().unwrap(),
        amount,
    );

    match result {
        Ok(signature) => {
            println!("Airdrop requested successfully. Signature: {:?}", signature);
            Ok(signature)
        }
        Err(err) => {
            println!("error: {:?}", err);
            Err("unable to airdrop {}".to_string())
        }
    }
}
  
