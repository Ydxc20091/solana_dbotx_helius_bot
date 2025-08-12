use async_trait::async_trait;
use thiserror::Error;

use crate::model::Order;

/// Errors from executors.
#[derive(Debug, Error)]
pub enum ExecError {
    #[error("{0}")]
    Other(String),
}

/// Trait implemented by order executors.
#[async_trait]
pub trait Executor {
    async fn execute(&self, order: Order) -> Result<(), ExecError>;
}

pub mod dry;
pub mod jupiter;
