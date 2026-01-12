use crate::providers::{CalculatorResult, ClipboardEntry, DesktopEntry, Gitmoji};

#[derive(Debug, Clone)]
pub enum ProviderResult {
    None,
    Gitmoji(Vec<Gitmoji>),
    Apps(Vec<DesktopEntry>),
    Calculator(CalculatorResult),
    Clipboard(Vec<ClipboardEntry>),
}
