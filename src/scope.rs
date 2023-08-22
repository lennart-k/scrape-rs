use std::{collections::HashMap, sync::Arc};

use crate::argument::Value;
use crate::object_path::ResolvePath;

pub trait Scope: ResolvePath + Sync + Send + 'static {
    fn get(&self, key: &str) -> Option<Arc<Value>>;
    fn set(&mut self, key: String, value: Arc<Value>) -> Option<Arc<Value>>;
}

pub type SimpleScope = HashMap<String, Arc<Value>>;

impl Scope for SimpleScope {
    fn get(&self, key: &str) -> Option<Arc<Value>> {
        self.resolve_path(key).ok()
    }
    fn set(&mut self, key: String, value: Arc<Value>) -> Option<Arc<Value>> {
        self.insert(key, value)
    }
}
