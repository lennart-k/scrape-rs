use std::collections::HashMap;
use std::ops::Deref;

use crate::argument::Value;
use crate::scope::Scope;
use crate::{argument::ArgumentMarker, layer::Layer};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug)]
pub struct DebugOutput {}

impl From<DebugOutput> for Value {
    fn from(_: DebugOutput) -> Self {
        Value::HashMap(HashMap::new())
    }
}

#[derive(Debug)]
pub struct DebugState {
    message: Value,
}
impl DebugState {
    async fn run(&self) -> Result<DebugOutput> {
        dbg!(&self.message);
        Ok(DebugOutput {})
    }
}

#[derive(Debug, Deserialize)]
pub struct DebugLayer {
    message: ArgumentMarker,
}
impl DebugLayer {
    fn build<S: Scope>(&self, scope: &S) -> Result<DebugState> {
        Ok(DebugState {
            message: self.message.get_value(scope)?.deref().clone(),
        })
    }
}

#[async_trait]
impl Layer for DebugLayer {
    async fn run<S: Scope>(&self, scope: &S) -> Result<Value> {
        let state = self.build(scope)?;
        let out = state.run().await?;
        Ok(out.into())
    }
}
