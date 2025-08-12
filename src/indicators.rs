/// Relative Strength Index.
pub fn rsi(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() <= period {
        return None;
    }
    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in prices.len() - period..prices.len() - 1 {
        let diff = prices[i + 1] - prices[i];
        if diff >= 0.0 {
            gains += diff;
        } else {
            losses -= diff;
        }
    }
    let avg_gain = gains / period as f64;
    let avg_loss = losses / period as f64;
    if avg_loss == 0.0 {
        return Some(100.0);
    }
    let rs = avg_gain / avg_loss;
    Some(100.0 - 100.0 / (1.0 + rs))
}

/// Compute moving average and Bollinger bands.
pub fn bollinger(prices: &[f64], period: usize, k: f64) -> Option<(f64, f64, f64)> {
    if prices.len() < period {
        return None;
    }
    let slice = &prices[prices.len() - period..];
    let mean = slice.iter().sum::<f64>() / period as f64;
    let var = slice.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / period as f64;
    let std = var.sqrt();
    Some((mean, mean - k * std, mean + k * std))
}

/// Relative volume compared to previous `lookback` candles.
pub fn rvol(volumes: &[f64], lookback: usize) -> Option<f64> {
    if volumes.len() < lookback + 1 {
        return None;
    }
    let last = *volumes.last()?;
    let avg = volumes[volumes.len() - lookback - 1..volumes.len() - 1]
        .iter()
        .sum::<f64>()
        / lookback as f64;
    if avg == 0.0 {
        return None;
    }
    Some(last / avg)
}
