use crate::argument::ArgumentMarker;
use crate::common::{Builder, GenericData};
use crate::layer::RunGeneric;
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug)]
pub struct FetchHttpOutput {
    url: String,
    body: String,
    status: u16,
}
impl From<FetchHttpOutput> for GenericData {
    fn from(value: FetchHttpOutput) -> Self {
        let mut data = Self::new();
        data.insert("url".to_string(), Arc::new(value.url.to_string()));
        data.insert("body".to_string(), Arc::new(value.body));
        data.insert("status".to_string(), Arc::new(value.status));
        data
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
    url: ArgumentMarker<String>,
}

impl Builder for FetchHttpLayer {
    type Args = FetchHttpState;
    fn build(&self, out: &GenericData) -> Result<Self::Args> {
        Ok(Self::Args {
            url: self.url.get_value(out)?.to_string(),
        })
    }
}

#[async_trait]
impl RunGeneric for FetchHttpLayer {
    async fn run_generic(&self, prev_out: &GenericData) -> Result<GenericData> {
        let args = self.build(prev_out)?;
        let out = args.run(&args).await?;
        dbg!(&out);
        Ok(out.into())
    }
}
