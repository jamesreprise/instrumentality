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
use crate::mdb::DBHandle;
use crate::subject::Subject;
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub response: String,
    pub user: User,
    pub subjects: Vec<Subject>,
    pub groups: Vec<Group>,
}

pub async fn login(key: Key, db: DBHandle) -> impl IntoResponse {
    let user: User = User::user_with_key(&key.key, &db).await.unwrap();
    let resp = LoginResponse {
        response: "OK".to_string(),
        user: user.clone(),
        subjects: user.subjects(&db).await.unwrap_or_default(),
        groups: user.groups(&db).await.unwrap_or_default(),
    };

    (StatusCode::OK, Json(resp))
}
