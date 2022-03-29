//! Catchers for Rocket.
//!
//! The default (and only) catcher is implemented here.
//!
//! Whenever we encounter an error, we return the fact it is an error, its error code
//! and potentially a reason using reason_lossy() from http::Status.

use rocket::http::Status;
use rocket::serde::json::Value;
use rocket::Request;
use serde_json::json;

#[catch(default)]
pub fn default_err(status: Status, _request: &Request) -> Value {
    json!({"response": "ERROR", "error_code": status.code, "text": status.reason_lossy()})
}
