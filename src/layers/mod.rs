use crate::common::GenericData;
use crate::layer::RunGeneric;
use anyhow::Result;
use async_trait::async_trait;
use fetch_http::FetchHttpLayer;
use regex_find::RegexFindLayer;
use serde::Deserialize;

pub mod fetch_http;
pub mod regex_find;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GenericLayer {
    FetchHttp(FetchHttpLayer),
    RegexFind(RegexFindLayer),
}

#[async_trait]
impl RunGeneric for GenericLayer {
    async fn run_generic(&self, out: &GenericData) -> Result<GenericData> {
        match self {
            GenericLayer::FetchHttp(builder) => builder.run_generic(out).await,
            GenericLayer::RegexFind(builder) => builder.run_generic(out).await,
        }
    }
}
