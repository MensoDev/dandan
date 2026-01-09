#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Key(char),
    Application,
}
