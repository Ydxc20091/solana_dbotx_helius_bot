mod config;
mod data;
mod exec;
mod indicators;
mod model;
mod risk;
mod signal;
mod util;

use clap::Parser;
use dotenvy::dotenv;

use config::{Cli, Config, ExecMode};
use exec::Executor;
use model::Order;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load `.env` files and parse CLI arguments, then load environment config.
    dotenv().ok();
    let cli = Cli::parse();
    let cfg = Config::from_env().expect("missing env config");

    // Fetch latest kline data.
    let end = util::now_ts() * 1000; // assume ms timestamp
    let klines = data::dbotx::fetch_klines(&cfg, end).await?;

    // Determine if entry signal exists.
    if let Some(sig) = signal::detect_signal(&klines, &cli) {
        let order = Order {
            price: sig.price,
            size: cfg.order_size_sol,
        };

        match cli.exec_mode {
            ExecMode::Dry => {
                let ex = exec::dry::DryExecutor;
                ex.execute(order).await?;
            }
            ExecMode::Live => {
                let ex = exec::jupiter::JupiterExecutor;
                ex.execute(order).await?;
            }
            ExecMode::Backtest => {
                println!("Backtest mode not implemented");
            }
        }
    } else {
        println!("No signal detected");
    }

    // Placeholder for risk management demonstration.
    if let Some(action) = risk::evaluate_risk(&klines) {
        println!("Risk action suggested: {:?}", action);
    }

    Ok(())
}
