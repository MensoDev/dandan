use crate::{engine::{Engine, ProviderType}, providers::{ApplicationProvider, CalculatorProvider, GitmojiProvider}};


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

        engine
    }
}
