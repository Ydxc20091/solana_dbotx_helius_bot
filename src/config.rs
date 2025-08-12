use std::env;

use clap::{Parser, ValueEnum};
use thiserror::Error;

/// Execution mode for the bot.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecMode {
    /// Do not send any transaction, only print intended actions.
    Dry,
    /// Submit transactions on chain using the configured RPC.
    Live,
    /// Run against historical data only.
    Backtest,
}

/// Command line arguments and environment configuration.
#[derive(Parser, Debug)]
#[command(version, about = "Solana DBotX Helius trading bot")]
pub struct Cli {
    /// Execution mode; can also be set via `EXEC_MODE` env.
    #[arg(long, env = "EXEC_MODE", default_value = "dry")]
    pub exec_mode: ExecMode,
    /// Whether RSI filter is enabled.
    #[arg(long, env = "RSI_FILTER", default_value_t = true)]
    pub rsi_filter: bool,
    /// Whether Bollinger band check is enabled.
    #[arg(long, env = "BB_FILTER", default_value_t = false)]
    pub bollinger_filter: bool,
}

/// Errors that can happen while building [`Config`].
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing env {0}")]
    MissingEnv(String),
}

fn env_var(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingEnv(key.to_string()))
}

/// Runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub dbotx_api_key: String,
    pub dbotx_kline_url: String,
    pub dbotx_chain: String,
    pub dbotx_pair: String,
    pub rpc_url: String,
    pub public_key: String,
    pub secure_code: String,
    pub base: String,
    pub quote: String,
    pub order_size_sol: f64,
}

impl Config {
    /// Load configuration from the environment.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            dbotx_api_key: env_var("DBOTX_API_KEY")?,
            dbotx_kline_url: env_var("DBOTX_KLINE_URL")?,
            dbotx_chain: env_var("DBOTX_CHAIN")?,
            dbotx_pair: env_var("DBOTX_PAIR")?,
            rpc_url: env_var("RPC_URL")?,
            public_key: env_var("PUBLIC_KEY")?,
            secure_code: env_var("SECURE_CODE")?,
            base: env_var("BASE")?,
            quote: env_var("QUOTE")?,
            order_size_sol: env_var("ORDER_SIZE_SOL")?
                .parse()
                .map_err(|_| ConfigError::MissingEnv("ORDER_SIZE_SOL".into()))?,
        })
    }
}
