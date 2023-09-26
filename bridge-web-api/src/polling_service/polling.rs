use sea_orm::DbErr;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use std::time::Duration;

pub async fn get_db_connection(config: &PollingConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(config.database_url.to_owned());
    opt.max_connections(config.sqlx_max_connections)
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
                .polling_sqlx_logging_level
                .parse::<log::LevelFilter>()
            {
                Ok(level) => level,
                Err(_) => log::LevelFilter::Info,
            },
        );

    Database::connect(opt).await
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PollingConfig {
    // bridge-polling-service
    pub database_url: String,
    pub on_chain_endpoint: String,
    pub sqlx_max_connections: u32,
    pub sqlx_min_connections: Option<u32>,
    pub sqlx_connect_timeout: Option<u64>,
    pub sqlx_idle_timeout: Option<u64>,
    pub sqlx_max_lifetime: Option<u64>,
    pub sqlx_logging: Option<bool>,
    pub polling_sqlx_logging_level: String,
    pub polling_sleep_secs: Option<u64>,
    pub polling_batch_transactions: Option<usize>,
    pub polling_batch_sleep_millis: Option<u64>,
    pub polling_daily_sleep_hms: Option<String>,
}
