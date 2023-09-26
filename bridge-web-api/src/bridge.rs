use async_trait::async_trait;
use rocket::serde::Deserialize;
use rocket::Config;
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Database};
use std::time::Duration;

#[derive(Database, Debug)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
    pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(_figment: &Figment) -> Result<Self, Self::Error> {
        let config = Config::figment().extract::<BridgeConfig>().unwrap();
        let mut options: ConnectOptions = config.database_url.into();
        options
            .max_connections(config.sqlx_max_connections)
            .min_connections(match config.sqlx_min_connections {
                Some(v) => v,
                None => 2,
            })
            .connect_timeout(Duration::from_secs(match config.sqlx_connect_timeout {
                Some(v) => v,
                None => 8,
            }))
            .idle_timeout(Duration::from_secs(match config.sqlx_idle_timeout {
                Some(v) => v,
                None => 8,
            }))
            .max_lifetime(Duration::from_secs(match config.sqlx_max_lifetime {
                Some(v) => v,
                None => 8,
            }))
            .sqlx_logging(match config.sqlx_logging {
                Some(v) => v,
                None => false,
            })
            .sqlx_logging_level(
                match config
                    .web_api_sqlx_logging_level
                    .parse::<log::LevelFilter>()
                {
                    Ok(level) => level,
                    Err(_) => log::LevelFilter::Info,
                },
            );

        let conn = sea_orm::Database::connect(options).await?;

        Ok(SeaOrmPool { conn })
    }

    fn borrow(&self) -> &Self::Connection {
        &self.conn
    }
}



#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct BridgeConfig {
    pub database_url: String,
    pub on_chain_endpoint: String,
    pub bridge_program_address: String,
    pub bridge_account_address: String,
    pub bridge_admin_address: String,
    pub bridge_account_token_mint: String,
    pub bridge_holding_wallet_owner: String,
    pub bridge_holding_wallet: String,
    pub gari_treassury_wallet: String,
    pub gari_faucet_account: String,
    pub gari_faucet_token_mint: String,
    pub faucet_admin_address: String,
    pub admin_token_account: String,
    pub rust_log: String,
    pub bridge_web_api_log: String,
    pub fee_payer_address: String,
    pub fee_payer_private_key: String,
    pub send_transaction_simulate: bool,
    pub admin_private_key: String,
    pub faucet_admin_private_key: String,
    pub bridge_platform_fee: u64,
    pub mantle_gas_fee: u64,
    pub sqlx_max_connections: u32,
    pub sqlx_min_connections: Option<u32>,
    pub sqlx_connect_timeout: Option<u64>,
    pub sqlx_idle_timeout: Option<u64>,
    pub sqlx_max_lifetime: Option<u64>,
    pub sqlx_logging: Option<bool>,
    pub web_api_sqlx_logging_level: String,
}

pub mod Instruction {
    pub const INIT_BRIDGE_STATE: [u8; 8] = [206, 176, 202, 18, 200, 209, 179, 108];
    pub const LOCK: [u8; 8] = [153, 150, 141, 190, 78, 115, 232, 80];
    pub const UNLOCK: [u8; 8] = [160, 102, 188, 56, 211, 160, 144, 113];
}
