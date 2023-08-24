use std::{collections::HashMap, sync::Arc};

use anyhow::{Error, Result};

use crate::argument::Value;
use crate::object_path::ResolvePath;

pub trait Scope: Sync + Send + 'static {
    fn get(&self, key: &str) -> Option<Arc<Value>>;
    fn set(&mut self, key: String, value: Arc<Value>) -> Option<Arc<Value>>;
}

impl<S: Scope> ResolvePath for S {
    fn resolve(&self, path: &str) -> Result<Arc<Value>> {
        let pattern = regex::Regex::new(r"^(?P<key>\w+)(\.(?P<rest>.+))?").unwrap();
        let cap = pattern
            .captures(path)
            .ok_or(Error::msg("invalid pattern"))?;
        let key = cap.name("key").unwrap().as_str();
        let out = self
            .get(key)
            .ok_or(Error::msg(format!("key does not exist: {key}")))?;

        if let Some(rest_match) = cap.name("rest") {
            return out.resolve(rest_match.as_str());
        }

        Ok(out)
    }
}

pub type SimpleScope = HashMap<String, Arc<Value>>;

impl Scope for SimpleScope {
    fn get(&self, key: &str) -> Option<Arc<Value>> {
        Some(self.get(key)?.clone())
    }
    fn set(&mut self, key: String, value: Arc<Value>) -> Option<Arc<Value>> {
        self.insert(key, value)
    }
}
