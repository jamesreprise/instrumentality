//! Route for registering as a new user for Instrumentality.
//!
//! The /register route is implemented here.
//!
//! This route registers a user for an API key, not a user/pass combo.
//!
//! A register request takes the form:
//! ```json
//! {
//!     "ref_code": String,
//!     "name": String,
//! }
//! ```

use crate::database::DBHandle;
use crate::response::{Error, RegisterResponse};
use crate::routes::invite::Referral;
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    ref_code: String,
    name: String,
}

#[derive(Debug)]
pub struct RegisterError;

// Invites can't be double used but we are double requesting with every attempt
// /register wrt invite_valid and use_invite.
pub async fn register(Json(req): Json<RegisterRequest>, db: DBHandle) -> impl IntoResponse {
    if invite_valid(&req, &db).await && username_not_taken(&req, &db).await {
        let result = register_user(&req, &db).await;
        match result {
            Ok(user) => Ok((StatusCode::OK, Json(RegisterResponse::new(user)))),
            _ => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::new("Internal server error.")),
            )),
        }
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(Error::new(
                "Either the invite is invalid or the username is taken.",
            )),
        ))
    }
}

async fn invite_valid(req: &RegisterRequest, db: &DBHandle) -> bool {
    let refs_coll: Collection<Referral> = db.collection("referrals");
    let result = refs_coll
        .find_one(
            doc! {"ref_code": req.ref_code.as_str(), "used" : false},
            None,
        )
        .await;
    matches!(result, Ok(Some(_)))
}

async fn username_not_taken(req: &RegisterRequest, db: &DBHandle) -> bool {
    let users_coll: Collection<User> = db.collection("users");
    let result = users_coll
        .find_one(doc! {"user": req.name.as_str()}, None)
        .await;
    matches!(result, Ok(None))
}

async fn register_user(req: &RegisterRequest, db: &DBHandle) -> Result<User, RegisterError> {
    let user = User::new(&req.name);
    let result = use_invite(&user, req, &db).await;
    if result.is_ok() {
        let users_coll: Collection<User> = db.collection("users");

        let result = users_coll.insert_one(&user, None).await;
        match result {
            Ok(_) => Ok(user),
            _ => Err(RegisterError),
        }
    } else {
        Err(RegisterError)
    }
}

async fn use_invite(
    user: &User,
    req: &RegisterRequest,
    db: &DBHandle,
) -> Result<Referral, RegisterError> {
    let refs_coll: Collection<Referral> = db.collection("referrals");
    let result = refs_coll
        .find_one_and_update(
            doc! {"ref_code": req.ref_code.as_str(), "used": false},
            doc! {"$set": {"used": true, "used_by": &user.uuid}},
            None,
        )
        .await
        .unwrap();
    match result {
        Some(entry) => Ok(entry),
        _ => Err(RegisterError),
    }
}
