use anyhow::Result;
use async_trait::async_trait;
use common::GenericData;
use fetch_http::FetchHttpLayer;
use layer::RunGeneric;
use regex_find::RegexFindLayer;
use serde::Deserialize;
use std::fs;

pub mod argument;
pub mod common;
mod fetch_http;
pub mod layer;
mod regex_find;

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
enum ConfigurableArgsBuilder {
    FetchHttp(FetchHttpLayer),
    RegexFind(RegexFindLayer),
}

#[async_trait]
impl RunGeneric for ConfigurableArgsBuilder {
    async fn run_generic(&self, out: &GenericData) -> Result<GenericData> {
        match self {
            ConfigurableArgsBuilder::FetchHttp(builder) => builder.run_generic(out).await,
            ConfigurableArgsBuilder::RegexFind(builder) => builder.run_generic(out).await,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let file_contents = fs::read_to_string("sample.yml")?;
    let layers: Vec<ConfigurableArgsBuilder> = serde_yaml::from_str(&file_contents)?;
    let mut prev_out = GenericData::new();
    for layer in layers {
        prev_out = layer.run_generic(&prev_out).await?;
    }
    Ok(())
}
