use crate::helpers::TestApp;

#[tokio::test]
async fn login_should_return_200_if_valid_credentials() {
    let app = TestApp::new().await;

    app.post_signup(&serde_json::json!({
        "email": "requires2fa@mail.com",
        "password": "password123",
        "requires2FA": false
    }))
    .await;

    let body = serde_json::json!({
        "email": "requires2fa@mail.com",
        "password": "password123",
    });

    let response = app.post_login(&body).await;
    assert_eq!(
        response.status().as_u16(),
        200,
        "Failed for input: {:?}",
        body
    );
}

#[tokio::test]
async fn login_should_return_206_if_2fa_is_required() {
    let app = TestApp::new().await;

    app.post_signup(&serde_json::json!({
        "email": "login@mail.com",
        "password": "password123",
        "requires2FA": true
    }))
    .await;

    let body = serde_json::json!({
        "email": "login@mail.com",
        "password": "password123",
    });

    let response = app.post_login(&body).await;
    assert_eq!(
        response.status().as_u16(),
        206,
        "Failed for input: {:?}",
        body
    );
}

#[tokio::test]
async fn login_should_return_400_if_invalid_inupt() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "invalid",
            "password": "password123",
        }),
        serde_json::json!({
            "email": "login@mail.com",
            "password": "pass"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn login_should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    app.post_signup(&serde_json::json!({
        "email": "requires2fa@mail.com",
        "password": "password123",
        "requires2FA": true
    }))
    .await;

    app.post_signup(&serde_json::json!({
        "email": "login@mail.com",
        "password": "password123",
        "requires2FA": false
    }))
    .await;

    let test_cases = [
        serde_json::json!({
            "email": "invalid@mail.com",
            "password": "password123",
        }),
        serde_json::json!({
            "email": "requires2fa@mail.com",
            "password": "invalid1234"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn login_should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "mail": "invalid@mail.com",
            "password": "password123",
        }),
        serde_json::json!({
            "email": "requires2fa@mail.com",
            "pass": "nope"
        }),
        serde_json::json!({
            "email": "requires2fa@mail.com",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
