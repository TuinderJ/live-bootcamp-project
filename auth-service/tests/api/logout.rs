use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email,
    utils::{constants::JWT_COOKIE_NAME, generate_auth_cookie},
};
use reqwest::Url;
use test_helpers::api_test;

#[api_test]
async fn should_return_200_if_valid_jwt_cookie() {
    let email = Email::parse(get_random_email()).expect("Couldn't parse email");
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    assert_eq!(app.post_logout().await.status().as_u16(), 200);
    assert!(app
        .banned_token_store
        .read()
        .await
        .contains_token(cookie.value().to_string())
        .await
        .unwrap());
}

#[api_test]
async fn should_return_401_if_token_is_already_banned() {
    let email = Email::parse(get_random_email()).expect("Couldn't parse email");
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    assert_eq!(app.post_logout().await.status().as_u16(), 200);

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    assert_eq!(app.post_logout().await.status().as_u16(), 401);
}

#[api_test]
async fn should_return_400_if_logout_called_twice() {
    let email = Email::parse(get_random_email()).expect("Couldn't parse email");
    let cookie = generate_auth_cookie(&email).unwrap();

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}={}; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME,
            cookie.value()
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    assert_eq!(app.post_logout().await.status().as_u16(), 200);
    assert_eq!(app.post_logout().await.status().as_u16(), 400);
}

#[api_test]
async fn should_return_400_if_jwt_cookie_is_missing() {}

#[api_test]
async fn should_return_401_if_invalid_token() {
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse(&app.address).expect("Failed to parse URL"),
    );

    assert_eq!(app.post_logout().await.status().as_u16(), 401);
}
