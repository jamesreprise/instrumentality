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

use crate::routes::invite::Referral;
use crate::user::User;

use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    ref_code: String,
    name: String,
}

#[derive(Debug)]
pub struct RegisterError;

// Invites can't be double used but we are double requesting with every attempt
// /register wrt invite_valid and use_invite.
#[post("/register", format = "json", data = "<req>", rank = 1)]
pub async fn register(req: Json<RegisterRequest>, db: &State<Database>) -> Value {
    if invite_valid(&req, db).await && username_not_taken(&req, db).await {
        let result = register_user(&req, db).await;
        match result {
            Ok(user) => json!({"response": "OK", "result": user}),
            _ => {
                json!({"error_code": Status::InternalServerError.code, "text": Status::InternalServerError.reason_lossy()})
            }
        }
    } else {
        json!({"response": "ERROR", "error_code": 401, "text": "Invalid invite or username taken."})
    }
}

async fn invite_valid(req: &Json<RegisterRequest>, db: &State<Database>) -> bool {
    let refs_coll: Collection<Referral> = db.collection("referrals");
    let result = refs_coll
        .find_one(
            doc! {"ref_code": req.ref_code.as_str(), "used" : false},
            None,
        )
        .await;
    matches!(result, Ok(Some(_)))
}

async fn username_not_taken(req: &Json<RegisterRequest>, db: &State<Database>) -> bool {
    let users_coll: Collection<User> = db.collection("users");
    let result = users_coll
        .find_one(doc! {"user": req.name.as_str()}, None)
        .await;
    matches!(result, Ok(None))
}

async fn register_user(
    req: &Json<RegisterRequest>,
    db: &State<Database>,
) -> Result<User, RegisterError> {
    let user = User::new(&req.name);
    let result = use_invite(&user, req, db).await;
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
    req: &Json<RegisterRequest>,
    db: &State<Database>,
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
