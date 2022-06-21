//! Routes for fetching user information.
//!
//! The /login route is implemented here.
//!
//! See endpoint documentation at <https://docs.berserksystems.com/endpoints/login/>.

use crate::database::DBHandle;
use crate::key::Key;
use crate::response::LoginResponse;
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn login(key: Key, db: DBHandle) -> impl IntoResponse {
    let user: User = User::with_key(&key.key, &db).await.unwrap();
    let subjects = User::subjects(&user, &db).await.unwrap_or_default();
    let groups = User::groups(&user, &db).await.unwrap_or_default();
    let resp = LoginResponse::new(user.clone(), subjects, groups);

    (StatusCode::OK, Json(resp))
}
