use anyhow::Result;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

// GenericOutput for services
pub type GenericOutput = HashMap<String, Arc<dyn Any + Send + Sync>>;

pub trait Builder {
    type Args;

    fn build(&self, out: &GenericOutput) -> Result<Self::Args>;
}
