use async_trait::async_trait;

use super::{ExecError, Executor};
use crate::model::Order;

/// Executor that only prints orders without sending them on chain.
pub struct DryExecutor;

#[async_trait]
impl Executor for DryExecutor {
    async fn execute(&self, order: Order) -> Result<(), ExecError> {
        println!("DRY | price: {} size: {}", order.price, order.size);
        Ok(())
    }
}
