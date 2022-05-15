//! Route for resetting an API key for Instrumentality.
//!
//! The /reset route is implemented here.
//!
//! See endpoint documentation at https://instrumentality.berserksystems.com/docs/reset/.

use crate::database::DBHandle;
use crate::key::Key;
use crate::response::{Error, ResetResponse};
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;

pub async fn reset(key: Key, db: DBHandle) -> impl IntoResponse {
    let new_key = User::new_key();
    let u_coll = db.collection::<User>("users");
    let result = u_coll
        .find_one_and_update(
            doc! {"key": &key.key},
            doc! { "$set": {"key": &new_key}},
            None,
        )
        .await;
    match result {
        Ok(Some(_)) => Ok((StatusCode::OK, Json(ResetResponse::new(new_key)))),
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Error::new(
                "Could not reset key. Your key remains the same. Please try again.",
            )),
        )),
    }
}
