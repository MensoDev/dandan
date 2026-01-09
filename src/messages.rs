use crate::engine::Engine;

#[derive(Debug, Clone)]
pub enum Message {
    EngineReady(Engine),
    InputChanged(String),
    NavigationDown,
    NavigationUp,
    ExecuteAction,
    Exit
}
