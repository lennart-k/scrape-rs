use self::debug_layer::DebugLayer;
use crate::{argument::Value, layer::Layer, scope::Scope};
use anyhow::Result;
use async_trait::async_trait;
use fetch_http::FetchHttpLayer;
use regex_find::RegexFindLayer;
use serde::Deserialize;

pub mod debug_layer;
pub mod fetch_http;
pub mod regex_find;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GenericLayer {
    FetchHttp(FetchHttpLayer),
    RegexFind(RegexFindLayer),
    Debug(DebugLayer),
}

#[async_trait]
impl Layer for GenericLayer {
    async fn run<S: Scope>(&self, context: &S) -> Result<Value> {
        match self {
            GenericLayer::FetchHttp(layer) => layer.run(context).await,
            GenericLayer::RegexFind(layer) => layer.run(context).await,
            GenericLayer::Debug(layer) => layer.run(context).await,
        }
    }
}
