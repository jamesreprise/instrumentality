//! API keys for authorisation.

use crate::user::User;

use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

pub struct Key {
    pub key: String,
}

#[derive(Debug)]
pub struct InvalidKeyError;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Key {
    type Error = InvalidKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        async fn valid(db: &State<Database>, key: String) -> bool {
            let users: Collection<User> = db.collection("users");
            // Vulnerable to nosql injection?
            // Very basic tests says no. Not convinced.
            matches!(
                users
                    .find_one(doc! {"key": key, "banned": false}, None)
                    .await,
                Ok(Some(_))
            )
        }

        let db = request
            .guard::<&State<Database>>()
            .await
            .succeeded()
            .unwrap();

        // This will still perform a lookup for key 'invalid'.
        let key = request.headers().get_one("x-api-key").unwrap_or("invalid");
        let result = request
            .local_cache_async(async { valid(db, key.to_string()).await })
            .await;

        match result {
            true => Outcome::Success(Key {
                key: key.to_string(),
            }),
            false => Outcome::Failure((Status::Unauthorized, InvalidKeyError)),
        }
    }
}
