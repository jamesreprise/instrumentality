//! Routes for fetching user information.
//!
//! The /login route is implemented here.
//!
//! You can login by calling /login with a valid API Key in your request
//! headers under "x-api-key".
//!
//! This returns user information if the given key is valid.

use crate::group::Group;
use crate::key::Key;
use crate::subject::Subject;
use crate::user::User;

use mongodb::Database;
use rocket::serde::json::Value;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub response: String,
    pub user: User,
    pub subjects: Vec<Subject>,
    pub groups: Vec<Group>,
}

#[get("/login")]
pub async fn login(key: Key, database: &State<Database>) -> Value {
    let user: User = User::user_with_key(&key.key, database).await.unwrap();
    let lr = LoginResponse {
        response: "OK".to_string(),
        user: user.clone(),
        subjects: user.subjects(database).await.unwrap_or_default(),
        groups: user.groups(database).await.unwrap_or_default(),
    };
    json!(
        {
            "response": lr.response,
            "user": lr.user,
            "subjects": lr.subjects,
            "groups": lr.groups,
        }
    )
}
