use serde::{Deserialize, Serialize};

/// Basic 1m kline representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    pub timestamp: i64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: f64,
}

/// Trading signal detected by the strategy.
#[derive(Debug, Clone)]
pub struct TradeSignal {
    pub price: f64,
    pub volume: f64,
}

/// Order representation used by executors.
#[derive(Debug, Clone)]
pub struct Order {
    pub price: f64,
    pub size: f64,
}
