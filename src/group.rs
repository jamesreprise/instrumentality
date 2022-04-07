//! Groups for organisitions of subjects.

use crate::key::Key;
use crate::routes::create::CreateData;
use crate::user::User;

use chrono::{DateTime, Utc};
use mongodb::Database;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub uuid: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String, // Created by a user.
    pub name: String,
    pub subjects: Vec<String>,
    pub description: Option<String>,
}

impl Group {
    pub async fn from_group_create(cs: CreateData, db: &State<Database>, key: Key) -> Option<Self> {
        match cs {
            CreateData::CreateGroup {
                name,
                subjects,
                description,
            } => Some(Group {
                uuid: Uuid::new_v4().to_string(),
                created_at: Utc::now(),
                created_by: User::user_with_key(&key.key, db).await.unwrap().uuid,
                name,
                subjects,
                description,
            }),
            _ => None,
        }
    }
}
