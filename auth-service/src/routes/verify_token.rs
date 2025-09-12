use crate::{domain::AuthAPIError, utils::auth::validate_token, AppState};
use axum::{extract::State, http::status::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

pub async fn verify_token(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<VerifyTokenRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let token = request.token;
    if validate_token(&token, state.banned_token_store)
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    (jar, Ok(StatusCode::OK))
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
