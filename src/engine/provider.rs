use std::fmt::Debug;

use crate::ProviderResult;

pub trait Provider: Debug + Send + Sync {
    fn search(&self, query: &str) -> Option<ProviderResult>;
    fn init(&mut self);
}
