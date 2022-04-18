//! Route for creating invites for registering users.
//!
//! The /invite route is implemented here.
//!
//! You request an invite for someone else by calling /invite with a valid
//! API Key in your request headers under "x-api-key".
//!
//! Below is an example referral:
//! ```json
//! {
//!     "created_by": "72c34ca4-d540-46b2-8ae3-53b8988c023b",
//!     "created_at": "2022-03-21T15:13:13Z",
//!     "code": "2FBFE4D99040DBDE144E6581CAFC5FE90DB79A7C625706D781FF5E161D843CFFEE842E3BA4E9C45E98FDA24FE489EDC21E452BEC47E0BC64B6E6176212C29F38",
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
pub struct Referral {
    created_by: String,
    created_at: DateTime<Utc>,
    code: String,
    used: bool,
    used_by: Option<String>,
}

impl Referral {
    pub fn new(created_by: String) -> Self {
        Self {
            created_by,
            created_at: Utc::now(),
            code: Self::new_code(),
            used: false,
            used_by: None,
        }
    }

    pub fn new_code() -> String {
        let refcode_bytes: &mut [u8] = &mut [0; 64];
        getrandom::getrandom(refcode_bytes).unwrap();
        let mut code = String::new();
        for b in refcode_bytes {
            write!(&mut code, "{:0>2X}", b).unwrap();
        }
        code
    }
}

#[get("/invite", rank = 1)]
pub async fn invite(key: Key, db: &State<Database>) -> Value {
    create_invite(key, db).await.unwrap()
}

async fn create_invite(key: Key, db: &State<Database>) -> Result<Value, InviteError> {
    let code = Referral::new_code();

    let refer_coll: Collection<Referral> = db.collection("referrals");
    refer_coll
        .insert_one(
            Referral::new(User::user_with_key(&key.key, db).await.unwrap().uuid),
            None,
        )
        .await
        .unwrap();

    Ok(json!({"response": "OK", "code": &code}))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_invite() {
        let referral = Referral::new("test".to_string());

        assert!(!referral.used);
        assert_eq!(referral.created_by, "test");
        assert_eq!(referral.used_by, None);
    }

    #[test]
    fn test_code() {
        let referral = Referral::new("test".to_string());

        let re = regex::Regex::new(r"^([A-F0-9])*$").unwrap();
        assert_eq!(referral.code.len(), 128);
        assert!(re.is_match(&referral.code));
    }
}
