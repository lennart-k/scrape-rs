use std::any::Any;
use std::sync::Arc;

pub trait Scope {
    fn get(&self, key: String) -> Option<&Arc<dyn Any + Send + Sync>>;
    fn set(
        &mut self,
        key: String,
        value: Arc<dyn Any + Send + Sync>,
    ) -> Option<Arc<dyn Any + Send + Sync>>;
}
