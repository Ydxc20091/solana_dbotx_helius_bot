use crate::{
    config::Cli,
    indicators::{bollinger, rsi, rvol},
    model::{Kline, TradeSignal},
};

/// Detect "volume dump" entry signal according to the default rules.
pub fn detect_signal(klines: &[Kline], cli: &Cli) -> Option<TradeSignal> {
    if klines.len() < 20 {
        return None;
    }

    let last = klines.last()?;
    let prev = &klines[klines.len() - 2];

    // Price drop
    let drop = (last.close - prev.close) / prev.close;
    if drop > -0.06 {
        return None;
    }

    // Relative volume
    let volumes: Vec<f64> = klines.iter().map(|k| k.volume).collect();
    if rvol(&volumes, 10)? < 2.5 {
        return None;
    }

    // RSI filter
    let closes: Vec<f64> = klines.iter().map(|k| k.close).collect();
    if cli.rsi_filter {
        if rsi(&closes, 14)? > 28.0 {
            return None;
        }
    }

    // Bollinger filter
    if cli.bollinger_filter {
        let (_, lower, _) = bollinger(&closes, 20, 2.0)?;
        if last.close > lower {
            return None;
        }
    }

    Some(TradeSignal {
        price: last.close,
        volume: last.volume,
    })
}
