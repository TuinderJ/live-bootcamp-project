use auth_service::{
    app_state::AppState,
    get_postgres_pool,
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient},
    utils::constants::{prod, DATABASE_URL},
    Application,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
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

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
