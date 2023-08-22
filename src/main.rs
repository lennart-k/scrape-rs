use anyhow::Result;
use layer::Layer;
use layers::GenericLayer;
use scope::Scope;
use scope::SimpleScope;
use std::fs;
use std::sync::Arc;

pub mod argument;
pub mod layer;
pub mod layers;
pub mod object_path;
pub mod scope;

#[tokio::main]
async fn main() -> Result<()> {
    let file_contents = fs::read_to_string("sample.yml")?;
    let layers: Vec<GenericLayer> = serde_yaml::from_str(&file_contents)?;
    let mut context: SimpleScope = SimpleScope::new();
    for layer in layers {
        let out = Arc::new(layer.run(&context).await?);
        context.set("previous".to_string(), out.clone());
    }
    Ok(())
}
