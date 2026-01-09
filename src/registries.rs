use crate::{engine::{Engine, ProviderType}, providers::GitmojiProvider};


impl Engine {
    pub async fn create_loaded() -> Engine {
        let mut engine = Engine::new();

        let mut gitmoji = GitmojiProvider::new();
        engine.init(&mut gitmoji);
        engine.register(ProviderType::Key(':'), gitmoji);

        engine
    }
}
