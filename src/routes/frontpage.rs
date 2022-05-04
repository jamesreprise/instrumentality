use crate::response::Ok;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn frontpage() -> impl IntoResponse {
    (StatusCode::OK, Json(Ok::new()))
}
