use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};
use axum::{extract::State, http::status::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = jar.get(JWT_COOKIE_NAME);
    if cookie.is_none() {
        return (jar, Err(AuthAPIError::MissingToken));
    }

    let cookie = cookie.unwrap();
    let token = cookie.value().to_owned();

    let banned_token_store = state.banned_token_store;

    if validate_token(token.as_ref(), banned_token_store.clone())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    if banned_token_store
        .write()
        .await
        .add_token(token)
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }
    let updated_jar = jar.remove(JWT_COOKIE_NAME);

    (updated_jar, Ok(StatusCode::OK))
}
