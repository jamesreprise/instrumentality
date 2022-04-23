use rocket::serde::json::Value;
use serde_json::json;

#[get("/")]
pub async fn frontpage() -> Value {
    json!(
        {"response": "OK"}
    )
}
