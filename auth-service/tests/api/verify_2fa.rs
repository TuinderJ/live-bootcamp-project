use crate::helpers::{get_random_email, TestApp};
use auth_service::{domain::Email, routes::LoginResponse};
use test_helpers::api_test;

#[api_test]
async fn should_return_200_with_correct_code() {
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true,
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201, "Failed to signup new user");

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(
        response.status().as_u16(),
        206,
        "Failed to get a response asking for 2FA"
    );

    let login_attempt_id = if let LoginResponse::TwoFactorAuth(val) = response
        .json::<LoginResponse>()
        .await
        .expect("Failed to parse the response from login.")
    {
        val.login_attempt_id
    } else {
        panic!();
    };
    let email = Email::parse(random_email.clone()).unwrap();
    let two_fa_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .unwrap()
        .1;

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code.as_ref().to_string(),
    });

    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 200);
}

#[api_test]
async fn should_return_400_with_invalid_input() {
    let test_case = serde_json::json!({
        "email": "valid@mail.com",
        "loginAttemptId": "14",
        "2FACode": "13",
    });

    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 400)
}

#[api_test]
async fn should_return_401_with_incorrect_code() {
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true,
    });

    assert_eq!(
        app.post_signup(&test_case).await.status().as_u16(),
        201,
        "Failed to signup new user"
    );

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
    });

    let response = app.post_login(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        206,
        "Failed to get a response asking for 2FA"
    );

    let login_attempt_id = if let LoginResponse::TwoFactorAuth(val) =
        response.json::<LoginResponse>().await.unwrap()
    {
        val.login_attempt_id
    } else {
        panic!();
    };

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": "123456".to_string(),
    });

    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 401);
}

#[api_test]
async fn should_return_401_if_same_token_is_used_twice() {
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
        "requires2FA": true,
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201, "Failed to signup new user");

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "password123",
    });

    let response = app.post_login(&test_case).await;

    assert_eq!(
        response.status().as_u16(),
        206,
        "Failed to get a response asking for 2FA"
    );

    let login_attempt_id = if let LoginResponse::TwoFactorAuth(val) = response
        .json::<LoginResponse>()
        .await
        .expect("Failed to parse the response from login.")
    {
        val.login_attempt_id
    } else {
        panic!();
    };
    let email = Email::parse(random_email.clone()).unwrap();
    let two_fa_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .unwrap()
        .1;

    let test_case = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code.as_ref().to_string(),
    });

    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 200);
    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 401);
}

#[api_test]
async fn should_return_422_with_invalid_input() {
    let random_email = get_random_email();

    let test_case = serde_json::json!({
        "email": &random_email,
        "password": "how_did_we_get_here",
    });

    assert_eq!(app.post_verify_2fa(&test_case).await.status().as_u16(), 422)
}
