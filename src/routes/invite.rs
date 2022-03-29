//! Invite route for Rocket.
//!
//! The /invite route is implemented here.
//!
//! You request an invite for someone else by calling /invite with a valid
//! API Key in your request headers under "x-api-key".
//!
//! A referral takes the form:
//! ```json
//! {
//!     "created_by": String,
//!     "created_at": DateTime<Utc>,
//!     "ref_code": String,
//!     "used": bool,
//!     "used_by": Option<String>,
//! }
//! ```
//!
//! Below is an example referral:
//! ```json
//! {
//!     "created_by": "72c34ca4-d540-46b2-8ae3-53b8988c023b",
//!     "created_at": "2022-03-21T15:13:13Z",
//!     "ref_code": "E3A2DD028183606CC894BB5C6B2CCC6F",
//!     "used": "false",
//!     "used_by": "None",
//! }
//! ```

use crate::key::Key;
use crate::user::User;

use chrono::{DateTime, Utc};
use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::serde::json::Value;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;

#[derive(Debug)]
pub struct InviteError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefEntry {
    created_by: String,
    created_at: DateTime<Utc>,
    ref_code: String,
    used: bool,
    used_by: Option<String>,
}

#[get("/invite", rank = 1)]
pub async fn invite(key: Key, db: &State<Database>) -> Value {
    create_invite(key, db).await.unwrap()
}

async fn create_invite(key: Key, db: &State<Database>) -> Result<Value, InviteError> {
    let refer_coll: Collection<RefEntry> = db.collection("referrals");
    let refcode_bytes: &mut [u8] = &mut [0; 64];
    getrandom::getrandom(refcode_bytes).unwrap();
    let mut ref_code = String::new();
    for b in refcode_bytes {
        write!(&mut ref_code, "{}", format!("{:X?}", b)).unwrap();
    }

    refer_coll
        .insert_one(
            &RefEntry {
                created_by: User::user_with_key(&key.key, db).await.unwrap().uuid,
                created_at: Utc::now(),
                ref_code: ref_code.clone(),
                used: false,
                used_by: None,
            },
            None,
        )
        .await
        .unwrap();

    Ok(json!({"response": "OK", "ref_code": ref_code}))
}
