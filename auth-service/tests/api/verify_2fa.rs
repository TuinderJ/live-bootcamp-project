use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_should_return_ui() {
    let app = TestApp::new().await;

    let response = app.get_route("verify-2fa").await;

    assert_eq!(response.status().as_u16(), 200);
}
