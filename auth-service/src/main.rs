use auth_service::{
    app_state::AppState,
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient},
    utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let app_state = AppState::new(
        Arc::new(RwLock::new(HashmapUserStore::new())),
        Arc::new(RwLock::new(HashsetBannedTokenStore::new())),
        Arc::new(RwLock::new(HashmapTwoFACodeStore::new())),
        Arc::new(RwLock::new(MockEmailClient)),
    );
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
