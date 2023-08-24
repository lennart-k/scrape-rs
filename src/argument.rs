use std::{collections::HashMap, sync::Arc};

use crate::{object_path::ResolvePath, scope::Scope};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    // Here order matters. u64 will be tried first, then f64
    U64(u64),
    I64(i64),
    F64(f64),
    HashMap(HashMap<String, Arc<Value>>),
    Vec(Vec<Arc<Value>>),
}

// TODO: For `as` casts enforce bounds!
impl TryFrom<Value> for u64 {
    type Error = anyhow::Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::U64(value) => Ok(value),
            Value::I64(value) => Ok(value as u64),
            _ => Err(Self::Error::msg("invalid type")),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl TryFrom<Value> for i64 {
    type Error = anyhow::Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::U64(value) => Ok(value as i64),
            Value::I64(value) => Ok(value),
            _ => Err(Self::Error::msg("invalid type")),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl TryFrom<Value> for f64 {
    type Error = anyhow::Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::F64(value) => Ok(value),
            Value::U64(value) => Ok(value as Self),
            Value::I64(value) => Ok(value as Self),
            _ => Err(Self::Error::msg("invalid type")),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl TryFrom<Value> for String {
    type Error = anyhow::Error;
    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        match value {
            Value::String(string) => Ok(string),
            Value::U64(int) => Ok(format!("{int}")),
            _ => Err(Self::Error::msg("invalid type")),
        }
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextVariable {
    pub key: String,
}
impl ContextVariable {
    pub fn get_value<S: Scope>(&self, scope: &S) -> Result<Arc<Value>> {
        Ok(scope.resolve(self.key.as_str())?.clone())
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgumentMarker {
    // Order matters, otherwise ContextVariables will simply be deserialized as Values
    ContextVariable(ContextVariable),
    Value(Arc<Value>),
}

impl ArgumentMarker {
    pub fn get_value<S: Scope>(&self, scope: &S) -> Result<Arc<Value>> {
        match &self {
            Self::Value(value) => Ok(value.clone()),
            Self::ContextVariable(ctx_var) => ctx_var.get_value(scope),
        }
    }
}
