use anyhow::Result;
use async_trait::async_trait;

use crate::common::GenericOutput;

#[async_trait]
pub trait RunGeneric {
    async fn run_generic(&self, out: &GenericOutput) -> Result<GenericOutput>;
}
