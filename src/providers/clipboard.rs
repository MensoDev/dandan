use serde::Deserialize;
use crate::{ProviderResult, engine::Provider};

#[derive(Debug)]
pub struct ClipboardProvider {
    entries: Option<Vec<ClipboardEntry>>
}

impl ClipboardProvider {
    pub fn new() -> Self {
        ClipboardProvider {
            entries: None
        }
    }
}

impl Provider for ClipboardProvider  {

    fn search(&self, query: &str) -> Option<ProviderResult> {

        if let Some(entries) = &self.entries {
            let r = entries
                .iter()
                .filter(|e| e.content.to_lowercase().contains(&query.to_lowercase()))
                .cloned()
                .collect();

            return Some(ProviderResult::Clipboard(r));
        }

        None
    }

    fn init(&mut self) {
        self.entries = crate::ClipboardUtils::list();
    }

}

#[derive(Debug, Clone, Deserialize)]
pub struct ClipboardEntry {
    pub id: usize,
    pub content: String
}
