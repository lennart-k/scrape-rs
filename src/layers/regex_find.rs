use crate::{
    common::{Builder, GenericData},
    layer::RunGeneric,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::sync::Arc;

use crate::argument::ArgumentMarker;

// output of the layer
#[derive(Debug)]
pub struct RegexFindOutput {
    needle: String,
}
// convert output to generic output for further usage
// TODO: make this derivable
impl From<RegexFindOutput> for GenericData {
    fn from(value: RegexFindOutput) -> Self {
        let mut data = Self::new();
        data.insert("needle".to_string(), Arc::new(value.needle.to_string()));
        data
    }
}

// The state of a layer, this can be evaluated to an output
#[derive(Debug)]
pub struct RegexFindState {
    pattern: String,
    input: String,
}
// evaluate the layer, all arguments are encapsulated in the state
impl RegexFindState {
    async fn run(&self) -> anyhow::Result<RegexFindOutput> {
        let pattern = regex::Regex::new(self.pattern.as_str())?;
        let needle = pattern.find(self.input.as_str()).unwrap();
        Ok(RegexFindOutput {
            needle: needle.as_str().to_string(),
        })
    }
}
// This is a layer, constructable by serde
// It is a spec of parameters needed to receive an output
#[derive(Deserialize, Debug)]
pub struct RegexFindLayer {
    pattern: ArgumentMarker<String>,
    input: ArgumentMarker<String>,
}

// Implement Builder to generate arguments from previous output
impl Builder for RegexFindLayer {
    type Args = RegexFindState;
    fn build(&self, prev_out: &GenericData) -> Result<RegexFindState> {
        Ok(Self::Args {
            pattern: self.pattern.get_value(prev_out)?.to_string(),
            input: self.input.get_value(prev_out)?.to_string(),
        })
    }
}
#[async_trait]
impl RunGeneric for RegexFindLayer {
    async fn run_generic(&self, prev_out: &GenericData) -> Result<GenericData> {
        let args = self.build(prev_out)?;
        let out = args.run().await?;
        dbg!(&out);
        Ok(out.into())
    }
}
