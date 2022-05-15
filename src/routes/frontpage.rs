//! Route for the front page.
//!
//! The / route is implemented here.
//!
//! See endpoint documentation at <https://docs.berserksystems.com/endpoints/frontpage/>.

use crate::response::Ok;
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn frontpage() -> impl IntoResponse {
    (StatusCode::OK, Json(Ok::new()))
}
