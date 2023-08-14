use crate::scope::Scope;
use anyhow::Result;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

// GenericOutput for services
pub type GenericData = HashMap<String, Arc<dyn Any + Send + Sync>>;

impl Scope for GenericData {
    fn get(&self, key: String) -> Option<&Arc<dyn Any + Send + Sync>> {
        HashMap::get(self, &key)
    }

    fn set(
        &mut self,
        key: String,
        value: Arc<dyn Any + Send + Sync>,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        self.insert(key, value)
    }
}

pub trait Builder {
    type Args;

    fn build(&self, out: &GenericData) -> Result<Self::Args>;
}
