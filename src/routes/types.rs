//! Route to get supported content and presence types.
//!
//! The /types route is implemented here.
//!
//! This returns the content/presence types this Instrumentality instance
//! accepts.

use crate::config::IConfig;

use rocket::serde::json::Value;
use rocket::State;
use serde_json::json;

#[get("/types")]
pub async fn types(iconfig: &State<IConfig>) -> Value {
    json!({"response": "OK", "content_types": iconfig.content_types, "presence_types": iconfig.presence_types})
}
