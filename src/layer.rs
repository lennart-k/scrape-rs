use anyhow::Result;
use async_trait::async_trait;

use crate::common::GenericData;

#[async_trait]
pub trait RunGeneric {
    async fn run_generic(&self, out: &GenericData) -> Result<GenericData>;
}
