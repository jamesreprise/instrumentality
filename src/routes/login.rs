//! Routes for fetching user information.
//!
//! The /login route is implemented here.
//!
//! See endpoint documentation at https://instrumentality.berserksystems.com/docs/login/.

use crate::database;
use crate::database::DBHandle;
use crate::group::Group;
use crate::key::Key;
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
    let user: User = database::user_with_key(&key.key, &db).await.unwrap();
    let resp = LoginResponse {
        response: "OK".to_string(),
        user: user.clone(),
        subjects: database::user_subjects(&user, &db)
            .await
            .unwrap_or_default(),
        groups: database::user_groups(&user, &db).await.unwrap_or_default(),
    };

    (StatusCode::OK, Json(resp))
}
