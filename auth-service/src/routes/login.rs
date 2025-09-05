use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = Email::parse(request.email);
    if email.is_err() {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }
    let email = email.unwrap();

    let password = Password::parse(request.password);
    if password.is_err() {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    }
    let password = password.unwrap();

    let user_store = state.user_store.write().await;

    if let Err(e) = user_store
        .validate_user(email.as_ref(), password.as_ref())
        .await
        .map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
            UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
            UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            _ => AuthAPIError::UnexpectedError,
        })
    {
        return (jar, Err(e));
    }

    let response = Json(LoginResponse {
        message: "Login successful".to_string(),
    });

    let auth_cookie = generate_auth_cookie(&email);
    if auth_cookie.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }
    let auth_cookie = auth_cookie.unwrap();

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::OK, response)))
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
