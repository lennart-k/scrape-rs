use crate::scope::Scope;
use crate::{argument::Value, layer::Layer};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::ops::Deref;
use std::{collections::HashMap, sync::Arc};

use crate::argument::ArgumentMarker;

// output of the layer
#[derive(Debug)]
pub struct RegexFindOutput {
    needle: String,
    groups: HashMap<String, String>,
}
// convert output to generic output for further usage
// TODO: make this derivable
impl From<RegexFindOutput> for Value {
    fn from(mut value: RegexFindOutput) -> Self {
        let mut data: HashMap<String, Arc<Value>> = HashMap::new();
        data.set(
            "needle".to_string(),
            Arc::new(Value::String(value.needle.to_string())),
        );
        let groups: HashMap<String, Arc<Value>> = HashMap::from_iter(
            value
                .groups
                .iter_mut()
                .map(|(key, value)| (key.clone(), Arc::new(value.clone().into()))),
        );
        data.set("groups".to_string(), Arc::new(Value::HashMap(groups)));
        Value::HashMap(data)
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
    async fn run(&self) -> Result<RegexFindOutput> {
        let pattern = regex::Regex::new(self.pattern.as_str())?;
        let re_match = pattern.find(self.input.as_str()).unwrap();
        let caps = pattern.captures(self.input.as_str()).unwrap();
        let mut named_groups: HashMap<String, String> = pattern
            .capture_names()
            .flat_map(|o| {
                o.and_then(|name| Some((name.to_string(), caps.name(name)?.as_str().to_string())))
            })
            .collect();
        for i in 0..caps.len() {
            named_groups.insert(i.to_string(), caps.get(i).unwrap().as_str().to_string());
        }
        Ok(RegexFindOutput {
            needle: re_match.as_str().to_string(),
            groups: named_groups,
        })
    }
}
// This is a layer, constructable by serde
// It is a spec of parameters needed to receive an output
#[derive(Deserialize, Debug)]
pub struct RegexFindLayer {
    pattern: ArgumentMarker,
    input: ArgumentMarker,
}

// Implement Builder to generate arguments from previous output
impl RegexFindLayer {
    fn build<S: Scope>(&self, scope: &S) -> Result<RegexFindState> {
        Ok(RegexFindState {
            pattern: self.pattern.get_value(scope)?.deref().clone().try_into()?,
            input: self.input.get_value(scope)?.deref().clone().try_into()?,
        })
    }
}

#[async_trait]
impl Layer for RegexFindLayer {
    async fn run<S: Scope>(&self, prev_out: &S) -> Result<Value> {
        let args = self.build(prev_out)?;
        let out = args.run().await?;
        dbg!(&out);
        Ok(out.into())
    }
}
