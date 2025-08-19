use axum::{http::status::StatusCode, response::IntoResponse};

pub async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
