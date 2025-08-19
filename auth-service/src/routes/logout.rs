use axum::{http::status::StatusCode, response::IntoResponse};

pub async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
