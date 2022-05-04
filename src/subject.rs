//! Subjects for organisation of profiles.

use crate::key::Key;
use crate::mdb::DBHandle;
use crate::routes::create::CreateData;
use crate::user::User;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub uuid: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub name: String,
    pub profiles: HashMap<String, Vec<String>>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subjects {
    pub data: Vec<Subject>,
}

// We're not using From/Into as currently functions in traits cannot be declared `async`
// Additionally, it's unclear whether From allows failure without extra plumbing.
impl Subject {
    pub async fn from_subject_create(cs: CreateData, db: &DBHandle, key: Key) -> Option<Self> {
        match cs {
            CreateData::CreateSubject {
                name,
                profiles,
                description,
            } => Some(Subject {
                uuid: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                created_by: User::user_with_key(&key.key, db).await.unwrap().uuid,
                name,
                profiles,
                description,
            }),
            _ => None,
        }
    }
}
