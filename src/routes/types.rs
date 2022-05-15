//! Route to get supported content and presence types.
//!
//! The /types route is implemented here.
//!
//! See endpoint documentation at https://instrumentality.berserksystems.com/docs/types/.

use crate::config::IConfig;
use crate::response::TypesResponse;

use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn types(config: IConfig) -> impl IntoResponse {
    let resp = TypesResponse::new(config.content_types, config.presence_types);

    (StatusCode::OK, Json(resp))
}
