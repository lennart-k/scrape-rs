use crate::argument::Value;
use anyhow::{Error, Result};
use std::{collections::HashMap, sync::Arc};

pub trait ResolvePath {
    fn resolve_path(&self, path: &str) -> Result<Arc<Value>>;
}

impl ResolvePath for HashMap<String, Arc<Value>> {
    fn resolve_path(&self, path: &str) -> Result<Arc<Value>> {
        let pattern = regex::Regex::new(r"^(?P<key>\w+)(\.(?P<rest>.+))?").unwrap();
        let cap = pattern
            .captures(path)
            .ok_or(Error::msg("invalid pattern"))?;

        let key = cap.name("key").unwrap().as_str();
        let out = self
            .get(key)
            .ok_or(Error::msg(format!("key does not exist: {key}")))?
            .clone();

        if let Some(rest_match) = cap.name("rest") {
            return out.resolve_path(rest_match.as_str());
        }

        Ok(out)
    }
}

impl ResolvePath for Value {
    fn resolve_path(&self, path: &str) -> Result<Arc<Value>> {
        match self {
            Value::HashMap(hash_map) => hash_map.resolve_path(path),
            _ => Err(Error::msg("invalid type")),
        }
    }
}
