//! Route to get supported content and presence types.
//!
//! The /types route is implemented here.
//!
//! This returns the content/presence types this Instrumentality instance
//! accepts.
//!
//! These take the form:
//! ```json
//! {
//!     "response": "OK",
//!     "content_types":
//!         {
//!          "platform1": ["content_type_1", "content_type_2", ..],
//!          "platform2": ["content_type_3"],
//!          "platform3": ["content_type_1", "content_type_5"]
//!         }
//!     "presence_types":
//!         {
//!          "platform1":["presence_type_1"],
//!          "platform4":["presence_type_2"],
//!          "platform2":["presence_type_3"]
//!         }
//! }
//! ```

use crate::config::IConfig;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct TypesResponse {
    response: String,
    content_types: HashMap<String, Vec<String>>,
    presence_types: HashMap<String, Vec<String>>,
}

pub async fn types(config: IConfig) -> impl IntoResponse {
    let resp = TypesResponse {
        response: "OK".to_string(),
        content_types: config.content_types,
        presence_types: config.presence_types,
    };

    (StatusCode::OK, Json(resp))
}
