
#[derive(Debug)]
pub enum Message {
    EngineReady,
    InputChanged(String),
    NavigationDown,
    NavigationUp,
    ExecuteAction,
    Exit
}
