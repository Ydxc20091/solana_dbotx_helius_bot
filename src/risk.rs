use crate::model::Kline;

/// Possible risk management actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskAction {
    /// Reduce existing position size.
    Reduce,
    /// Close the position entirely.
    Exit,
}

/// Evaluate open position risk based on the latest market data.
///
/// The current implementation is a placeholder and always returns `None`.
/// Proper risk controls such as structure break or time-based exit can be
/// implemented following the project specification.
pub fn evaluate_risk(_klines: &[Kline]) -> Option<RiskAction> {
    None
}
