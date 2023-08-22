use crate::argument::Value;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use crate::argument::ArgumentMarker;
use crate::layer::Layer;
use crate::scope::Scope;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug)]
pub struct FetchHttpOutput {
    url: String,
    body: String,
    status: u16,
}

impl From<FetchHttpOutput> for Value {
    fn from(value: FetchHttpOutput) -> Self {
        let mut data: HashMap<String, Arc<Value>> = HashMap::new();
        data.insert(
            "url".to_string(),
            Arc::new(Value::String(value.url.to_string())),
        );
        data.insert("body".to_string(), Arc::new(Value::String(value.body)));
        data.insert("status".into(), Arc::new(Value::U64(value.status as u64)));
        Value::HashMap(data)
    }
}

#[derive(Debug)]
pub struct FetchHttpState {
    url: String,
}
impl FetchHttpState {
    async fn run(&self, args: &FetchHttpState) -> Result<FetchHttpOutput> {
        let response = reqwest::get(&args.url).await?;
        Ok(FetchHttpOutput {
            url: args.url.to_string(),
            status: response.status().into(),
            body: response.text().await?,
        })
    }
}

// This is a deserializable builder for the layer arguments
//
// It is used to allow argument parameterisation
#[derive(Deserialize, Debug)]
pub struct FetchHttpLayer {
    url: ArgumentMarker,
}

impl FetchHttpLayer {
    fn build<S: Scope>(&self, scope: &S) -> Result<FetchHttpState> {
        Ok(FetchHttpState {
            url: self.url.get_value(scope)?.deref().clone().try_into()?,
        })
    }
}

#[async_trait]
impl Layer for FetchHttpLayer {
    async fn run<S: Scope>(&self, context: &S) -> Result<Value> {
        let args = self.build(context)?;
        let out = args.run(&args).await?;
        Ok(out.into())
    }
}
