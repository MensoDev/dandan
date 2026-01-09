mod provider;
mod provider_type;

use std::{collections::HashMap, sync::Arc};
use crate::ProviderResult;

pub use provider::Provider;
pub use provider_type::ProviderType;

#[derive(Debug, Clone)]
pub struct Engine {
    providers: HashMap<ProviderType, Arc<dyn Provider + Send>>,
    keys: Vec<ProviderType>
}

impl Engine {
    pub fn new() -> Self {
        Engine { providers: HashMap::new(), keys: vec![] }
    }

    pub fn init<P: Provider>(&self, provider: &mut P) {
        provider.init();
    }

    pub fn register<P: Provider + 'static>(&mut self, pt: ProviderType, provider: P) {
        self.keys.push(pt);
        self.providers.insert(pt, Arc::new(provider));
    }

    pub fn search(&self, query: &str) -> Option<ProviderResult> {
        let (key, query) = self.try_take_key(query.trim_start())?;

        self.providers
            .get(&key)
            .map(|p| p.search(&query))?
    }

    fn try_take_key(&self, query: &str) -> Option<(ProviderType, String)> {
        let first_char = query.trim_start().chars().next()?;

        for key in &self.keys {
            if let ProviderType::Key(token) = key {
                if *token == first_char {
                    return Some((key.clone(), query[1..].to_string()));
                }
            }
        }

        Some((ProviderType::Application, query.to_string()))
    }
}

