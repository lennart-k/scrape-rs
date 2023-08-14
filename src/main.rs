use anyhow::Result;
use common::GenericData;
use layer::RunGeneric;
use layers::GenericLayer;
use std::fs;

pub mod argument;
pub mod common;
pub mod layer;
pub mod layers;
pub mod scope;

#[tokio::main]
async fn main() -> Result<()> {
    let file_contents = fs::read_to_string("sample.yml")?;
    let layers: Vec<GenericLayer> = serde_yaml::from_str(&file_contents)?;
    let mut prev_out = GenericData::new();
    for layer in layers {
        prev_out = layer.run_generic(&prev_out).await?;
    }
    Ok(())
}
