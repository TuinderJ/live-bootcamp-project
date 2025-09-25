use crate::helpers::TestApp;
use auth_service::{
    domain::{Email, LoginAttemptId},
    routes::TwoFactorAuthResponse,
};
use test_helpers::api_test;

#[api_test]
async fn should_return_200_if_2fa_is_not_required() {
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

#[api_test]
async fn should_return_206_if_2fa_is_required() {
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

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());
    assert_eq!(
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse("login@mail.com".to_string()).expect("Failed to parse email"))
            .await
            .unwrap()
            .0,
        LoginAttemptId::parse(json_body.login_attempt_id).unwrap()
    );
}

#[api_test]
async fn should_return_400_if_invalid_inupt() {
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

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
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

#[api_test]
async fn should_return_422_if_malformed_credentials() {
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
        assert_eq!(
            app.post_login(test_case).await.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
