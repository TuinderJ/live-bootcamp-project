use crate::helpers::{get_random_email, TestApp};
use auth_service::{domain::Email, utils::auth::generate_auth_cookie};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({"invalid": "test"}),
        serde_json::json!({"tok": 13}),
    ];

    for test_case in test_cases.iter() {
        assert_eq!(
            app.post_verify_token(test_case).await.status().as_u16(),
            422
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    assert_eq!(
        app.post_verify_token(&serde_json::json!({"token": "1234"}))
            .await
            .status()
            .as_u16(),
        401
    );
}

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let email = Email::parse(random_email).expect("Failed to parse Email");

    let token = generate_auth_cookie(&email).expect("Failed to generate token");

    assert_eq!(
        app.post_verify_token(&serde_json::json!({"token": token.value()}))
            .await
            .status()
            .as_u16(),
        200
    );
}
