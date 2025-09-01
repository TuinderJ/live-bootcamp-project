use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStoreError},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.write().await;

    user_store
        .validate_user(email.as_ref(), password.as_ref())
        .await
        .map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
            UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
            UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            _ => AuthAPIError::UnexpectedError,
        })?;

    let response = Json(LoginResponse {
        message: "Login successful".to_string(),
    });

    Ok((StatusCode::OK, response))
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
