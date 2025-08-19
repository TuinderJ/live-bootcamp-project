use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_should_return_ui() {
    let app = TestApp::new().await;

    let response = app.get_route("verify-token").await;

    assert_eq!(response.status().as_u16(), 200);
}
