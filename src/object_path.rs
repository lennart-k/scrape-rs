use crate::argument::Value;
use anyhow::{Error, Result};
use std::sync::Arc;

pub trait ResolvePath {
    fn resolve(&self, path: &str) -> Result<Arc<Value>>;
}

impl ResolvePath for Value {
    fn resolve(&self, path: &str) -> Result<Arc<Value>> {
        match self {
            Value::HashMap(hash_map) => hash_map.resolve(path),
            _ => Err(Error::msg("invalid type")),
        }
    }
}
