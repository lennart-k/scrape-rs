use anyhow::Result;
use async_trait::async_trait;

use crate::{argument::Value, scope::Scope};

#[async_trait]
pub trait Layer {
    async fn run<S: Scope>(&self, context: &S) -> Result<Value>;
}
