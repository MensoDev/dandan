use crate::{engine::{Engine, ProviderType}, providers::{ApplicationProvider, CalculatorProvider, ClipboardProvider, GitmojiProvider}};


impl Engine {
    pub async fn create_loaded() -> Engine {
        let mut engine = Engine::new();

        let mut apps = ApplicationProvider::new();
        engine.init(&mut apps);
        engine.register(ProviderType::Application, apps);

        let mut gitmoji = GitmojiProvider::new();
        engine.init(&mut gitmoji);
        engine.register(ProviderType::Key(':'), gitmoji);

        let mut calculator = CalculatorProvider::new();
        engine.init(&mut calculator);
        engine.register(ProviderType::Key('='), calculator);

        let mut clipboard = ClipboardProvider::new();
        engine.init(&mut clipboard);
        engine.register(ProviderType::Key(';'), clipboard);

        engine
    }
}
