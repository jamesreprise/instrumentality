//! Routes for fetching user information.
//!
//! The /login route is implemented here.
//!
//! You can login by calling /login with a valid API Key in your request
//! headers under "x-api-key".
//!
//! This returns user information if the given key is valid.

use crate::key::Key;
use crate::user::User;

use mongodb::Database;
use rocket::serde::json::Value;
use rocket::State;
use serde_json::json;

#[get("/login")]
pub async fn login(key: Key, database: &State<Database>) -> Value {
    let user: User = User::user_with_key(&key.key, database).await.unwrap();
    json!(
        {
            "response": "OK",
            "user": &user,
            "subjects": &user.subjects(database).await.unwrap_or(Vec::new()),
            "groups": &user.groups(database).await.unwrap_or(Vec::new()),
        }
    )
}
