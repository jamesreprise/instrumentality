use crate::response::Error;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn default() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(Error::new("Not found.")))
}
