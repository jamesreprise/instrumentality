//! Route for creating invites for registering users.
//!
//! The /invite route is implemented here.
//!
//! See endpoint documentation at <https://docs.berserksystems.com/endpoints/invite/>.

use crate::database::{self, DBHandle};
use crate::key::Key;
use crate::response::{Error, InviteResponse};

use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
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

pub async fn invite(key: Key, db: DBHandle) -> impl IntoResponse {
    create_invite(key, &db).await
}

async fn create_invite(
    key: Key,
    db: &DBHandle,
) -> Result<(StatusCode, Json<InviteResponse>), (StatusCode, Json<Error>)> {
    let code = Referral::new_code();

    let refer_coll: Collection<Referral> = db.collection("referrals");
    refer_coll
        .insert_one(
            Referral::new(database::user_with_key(&key.key, db).await.unwrap().uuid),
            None,
        )
        .await
        .unwrap();

    Ok((StatusCode::OK, Json(InviteResponse::new(code))))
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
