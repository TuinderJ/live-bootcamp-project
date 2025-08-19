use crate::helpers::TestApp;

#[tokio::test]
async fn login_should_return_ui() {
    let app = TestApp::new().await;

    let response = app.get_route("login").await;

    assert_eq!(response.status().as_u16(), 200);
}
