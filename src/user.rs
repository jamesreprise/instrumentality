//! Basic user concepts for Instrumentality.

use crate::subject::Subject;
use crate::{database::DBHandle, group::Group};

use mongodb::{bson::doc, Collection, Cursor};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use tokio_stream::StreamExt;
use uuid::Uuid;

#[derive(Eq, PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub key: String,
    pub banned: bool,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name: name.to_string(),
            key: Self::new_key(),
            banned: false,
        }
    }

    pub fn new_key() -> String {
        let key_bytes: &mut [u8] = &mut [0; 32];
        getrandom::getrandom(key_bytes).unwrap();
        let mut key = String::new();
        for b in key_bytes {
            write!(&mut key, "{:0>2X}", b).unwrap();
        }
        key
    }

    pub async fn user_with_key(key: &str, db: &DBHandle) -> Option<User> {
        let users_coll: Collection<User> = db.collection("users");
        let result = users_coll.find_one(doc! {"key": key}, None).await.unwrap();
        result
    }

    pub async fn subjects(&self, db: &DBHandle) -> Option<Vec<Subject>> {
        let subj_coll: Collection<Subject> = db.collection("subjects");
        let cursor: Cursor<Subject> = subj_coll
            .find(doc! {"created_by": &self.uuid}, None)
            .await
            .unwrap();

        let results: Vec<Result<Subject, mongodb::error::Error>> = cursor.collect().await;
        let subjects: Vec<Subject> = results.into_iter().map(|d| d.unwrap()).collect();
        if subjects.is_empty() {
            None
        } else {
            Some(subjects)
        }
    }

    pub async fn groups(&self, db: &DBHandle) -> Option<Vec<Group>> {
        let group_coll: Collection<Group> = db.collection("groups");
        let cursor: Cursor<Group> = group_coll
            .find(doc! {"created_by": &self.uuid}, None)
            .await
            .unwrap();

        let results: Vec<Result<Group, mongodb::error::Error>> = cursor.collect().await;
        let groups: Vec<Group> = results.into_iter().map(|d| d.unwrap()).collect();
        if groups.is_empty() {
            None
        } else {
            Some(groups)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_user() {
        let user = User::new("test");

        assert!(!user.banned);
        assert_eq!(user.name, "test");
    }

    #[test]
    fn test_key() {
        let user = User::new("test");
        let re = regex::Regex::new(r"^([A-F0-9])*$").unwrap();

        assert_eq!(user.key.len(), 64);
        assert!(re.is_match(&user.key));
    }
}
