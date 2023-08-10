use crate::common::GenericOutput;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousOutput {
    key: String,
}

impl<'a> PreviousOutput {
    pub fn get_value<T: 'static>(&self, out: &'a GenericOutput) -> Result<&'a T> {
        out.get(&self.key)
            .ok_or(Error::msg(format!("invalid key: {}", self.key)))?
            .downcast_ref::<T>()
            .ok_or(Error::msg(format!("invalid type for key: {}", self.key)))
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgumentMarker<T> {
    Value(T),
    PreviousOutput(PreviousOutput),
}

impl<'a, T: 'static> ArgumentMarker<T> {
    pub fn get_value(&'a self, out: &'a GenericOutput) -> Result<&'a T> {
        match &self {
            Self::Value(value) => Ok(value),
            Self::PreviousOutput(prev) => prev.get_value(out),
        }
    }
}