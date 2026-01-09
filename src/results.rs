use crate::providers::Gitmoji;

#[derive(Debug, Clone)]
pub enum ProviderResult {
    None,
    Gitmoji(Vec<Gitmoji>)
}
