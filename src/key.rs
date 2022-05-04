//! API keys for authorisation.

use crate::mdb::DBHandle;
use crate::response::Error;
use crate::user::User;

use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json};
use mongodb::bson::doc;
use mongodb::Collection;

pub struct Key {
    pub key: String,
}

async fn user_exists_and_not_banned(db: &DBHandle, key: &str) -> bool {
    let users: Collection<User> = db.collection("users");
    // Vulnerable to nosql injection?
    // Very basic tests says no. Not convinced.
    users
        .find_one(doc! {"key": key, "banned": false}, None)
        .await
        .unwrap()
        .is_some()
}

#[async_trait]
impl<B: Send> FromRequest<B> for Key {
    type Rejection = Response;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let db = request.extensions().get::<DBHandle>().unwrap();

        // This will still perform a lookup for key 'invalid'.
        let key = request.headers().get("x-api-key");
        match key {
            Some(key) => {
                let key = key.to_str().unwrap();
                let result = user_exists_and_not_banned(&db, key).await;

                match result {
                    true => Ok(Key {
                        key: key.to_string(),
                    }),
                    false => Err(
                        (StatusCode::UNAUTHORIZED, Json(Error::new("Unauthorized.")))
                            .into_response(),
                    ),
                }
            }
            None => {
                Err((StatusCode::UNAUTHORIZED, Json(Error::new("Unauthorized."))).into_response())
            }
        }
    }
}
