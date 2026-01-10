use crate::providers::{DesktopEntry, Gitmoji};

#[derive(Debug, Clone)]
pub enum ProviderResult {
    None,
    Gitmoji(Vec<Gitmoji>),
    Apps(Vec<DesktopEntry>)
}
