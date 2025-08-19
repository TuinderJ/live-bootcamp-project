use axum::serve::Serve;
use axum::{response::Html, routing::get, Router};
use std::error::Error;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/hello", get(hello_handler))
            .route("/signup", get(signup_handler))
            .route("/login", get(login_handler))
            .route("/logout", get(logout_handler))
            .route("/verify-2fa", get(verify_2fa_handler))
            .route("/verify-token", get(verify_token_handler));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World! I'm working with Rust!</h1>")
}

async fn signup_handler() -> Html<&'static str> {
    Html("")
}

async fn login_handler() -> Html<&'static str> {
    Html("")
}

async fn logout_handler() -> Html<&'static str> {
    Html("")
}

async fn verify_2fa_handler() -> Html<&'static str> {
    Html("")
}

async fn verify_token_handler() -> Html<&'static str> {
    Html("")
}
