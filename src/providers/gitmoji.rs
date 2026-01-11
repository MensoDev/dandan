use std::fs;
use serde::Deserialize;
use crate::{ProviderResult, engine::Provider};

#[derive(Debug)]
pub struct GitmojiProvider {
    gitmojis: Option<Vec<Gitmoji>>
}

impl GitmojiProvider {
    pub fn new() -> Self {
        GitmojiProvider {
            gitmojis: None
        }
    }
}

impl Provider for GitmojiProvider  {

    fn search(&self, query: &str) -> Option<ProviderResult> {

        if let Some(emojis) = &self.gitmojis {
            let r = emojis
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&query.to_lowercase()) || e.description.to_lowercase().contains(&query.to_lowercase()))
                .cloned()
                .collect();

            return Some(ProviderResult::Gitmoji(r));
        }

        None
    }

    fn init(&mut self) {
        if let Some(path) = crate::PathUtils::get_config_path("gitmojis.json") {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(emojis) = serde_json::from_str(&content) {
                    self.gitmojis = Some(emojis);
                }
            }
        }
    }

}

#[derive(Debug, Clone, Deserialize)]
pub struct Gitmoji {
    pub emoji: String,
    pub entity: String,
    pub code: String,
    pub description: String,
    pub name: String,
    pub semver: Option<String>
}
