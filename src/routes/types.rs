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
use crate::response::TypesResponse;

use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn types(config: IConfig) -> impl IntoResponse {
    let resp = TypesResponse::new(config.content_types, config.presence_types);

    (StatusCode::OK, Json(resp))
}
