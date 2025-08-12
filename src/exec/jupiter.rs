use async_trait::async_trait;

use super::{ExecError, Executor};
use crate::model::Order;

/// Placeholder executor that would use Jupiter/Raydium for real trades.
pub struct JupiterExecutor;

#[async_trait]
impl Executor for JupiterExecutor {
    async fn execute(&self, _order: Order) -> Result<(), ExecError> {
        // TODO: integrate with on-chain swap infrastructure.
        println!("Jupiter executor not implemented");
        Ok(())
    }
}
