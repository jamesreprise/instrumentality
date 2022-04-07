//! Basic user concepts for Instrumentality.

use crate::group::Group;
use crate::rocket::futures::StreamExt;
use crate::subject::Subject;

use mongodb::{bson::doc, Collection, Cursor, Database};
use rocket::State;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub user: String,
    pub key: String,
    pub banned: bool,
}

impl User {
    pub fn new(user: &str) -> Self {
        let key_bytes: &mut [u8] = &mut [0; 32];
        getrandom::getrandom(key_bytes).unwrap();
        let mut key = String::new();
        for b in key_bytes {
            write!(&mut key, "{}", format!("{:X?}", b)).unwrap();
        }

        User {
            uuid: Uuid::new_v4().to_string(),
            user: user.to_string(),
            key,
            banned: false,
        }
    }

    pub async fn user_with_key(key: &str, database: &State<Database>) -> Option<User> {
        let users_coll: Collection<User> = database.collection("users");
        let result = users_coll.find_one(doc! {"key": key}, None).await.unwrap();
        result
    }

    pub async fn subjects(self: &Self, database: &State<Database>) -> Option<Vec<Subject>> {
        let subj_coll: Collection<Subject> = database.collection("subjects");
        let cursor: Cursor<Subject> = subj_coll
            .find(doc! {"created_by": &self.uuid}, None)
            .await
            .unwrap();

        let results: Vec<Result<Subject, mongodb::error::Error>> = cursor.collect().await;
        let subjects: Vec<Subject> = results.into_iter().map(|d| d.unwrap()).collect();
        if subjects.len() == 0 {
            None
        } else {
            Some(subjects)
        }
    }

    pub async fn groups(self: &Self, database: &State<Database>) -> Option<Vec<Group>> {
        let group_coll: Collection<Group> = database.collection("groups");
        let cursor: Cursor<Group> = group_coll
            .find(doc! {"created_by": &self.uuid}, None)
            .await
            .unwrap();

        let results: Vec<Result<Group, mongodb::error::Error>> = cursor.collect().await;
        let groups: Vec<Group> = results.into_iter().map(|d| d.unwrap()).collect();
        if groups.len() == 0 {
            None
        } else {
            Some(groups)
        }
    }
}
