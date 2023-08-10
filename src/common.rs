use anyhow::Result;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

// GenericOutput for services
pub type GenericData = HashMap<String, Arc<dyn Any + Send + Sync>>;

pub trait Builder {
    type Args;

    fn build(&self, out: &GenericData) -> Result<Self::Args>;
}
