//! Route for updating subjects and groups.
//!
//! The update route is implemented here.
//!
//! When you update a subject or a group, you MUST post all their information.
//! No old data is merged.
//!
//! An example update request is given below:
//! ```json
//! {
//!     "uuid": "<UUIDv4>",
//!     "name": "Subject1",
//!     "profiles":
//!         {
//!          "twitter": ["subject1", "subject1_private"],
//!          "instagram": ["subject1pics", "subject1pics_private"]
//!         },
//!     "description": "Everyone's favourite subject?"
//! }
//! ```
//! Which yields the response:
//! ```json
//! {
//!     "response": "OK"
//! }

use crate::group::Group;
use crate::key::Key;
use crate::routes::queue;
use crate::subject::*;
use crate::user::User;

use mongodb::{bson, Collection};
use mongodb::{bson::doc, Database};
use rocket::serde::json::{Json, Value};
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UpdateData {
    UpdateSubject {
        uuid: String,
        name: String,
        profiles: HashMap<String, Vec<String>>,
        description: Option<String>,
    },
    UpdateGroup {
        uuid: String,
        name: String,
        subjects: Vec<String>,
        description: Option<String>,
    },
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateData>, db: &State<Database>, key: Key) -> Value {
    let data = data.into_inner();
    match &data {
        UpdateData::UpdateSubject { .. } => update_subject(&data, db, &key).await,
        UpdateData::UpdateGroup { .. } => update_group(&data, db, &key).await,
    }
}

async fn update_subject(data: &UpdateData, db: &State<Database>, key: &Key) -> Value {
    let (uuid, name, profiles, description) = match data {
        UpdateData::UpdateSubject {
            uuid,
            name,
            profiles,
            description,
        } => (uuid, name, profiles, description),
        _ => panic!("Expected UpdateSubject."),
    };
    let req_uuid = User::user_with_key(&key.key, db).await.unwrap().uuid;
    let subj_coll: Collection<Subject> = db.collection("subjects");
    if let Ok(Some(subject)) = subj_coll
        .find_one(doc! {"uuid": &uuid, "created_by": &req_uuid}, None)
        .await
    {
        let mut old_profiles: Vec<(&String, &String)> = Vec::new();
        for platform in subject.profiles.keys() {
            for id in subject.profiles.get(platform).unwrap() {
                old_profiles.push((platform, id));
            }
        }
        let mut new_profiles: Vec<(&String, &String)> = Vec::new();
        for platform in profiles.keys() {
            for id in profiles.get(platform).unwrap() {
                new_profiles.push((platform, id));
            }
        }

        let removed_profiles: Vec<_> = old_profiles
            .iter()
            .filter(|x| new_profiles.contains(x))
            .collect();
        let added_profiles: Vec<_> = new_profiles
            .iter()
            .filter(|x| old_profiles.contains(x))
            .collect();

        for (platform, id) in added_profiles {
            queue::add_queue_item(id, platform, db).await.unwrap();
        }
        for (platform, id) in removed_profiles {
            queue::remove_queue_item(id, platform, db).await.unwrap();
        }

        subj_coll
            .update_one(
                doc! {"uuid": &uuid, "created_by": &req_uuid},
                doc! {"$set": {"name": name, "profiles": bson::to_bson(&profiles).unwrap(), "description": description}},
                None,
            )
            .await
            .unwrap();
        json!({ "response" : "OK"})
    } else {
        json!({ "response" : "ERROR", "text": "Subject does not exist or was not created by user with given key."})
    }
}

async fn update_group(data: &UpdateData, db: &State<Database>, key: &Key) -> Value {
    let (uuid, name, subjects, description) = match data {
        UpdateData::UpdateGroup {
            uuid,
            name,
            subjects,
            description,
        } => (uuid, name, subjects, description),
        _ => panic!("Expected UpdateGroup."),
    };
    let req_uuid = User::user_with_key(&key.key, db).await.unwrap().uuid;
    let group_coll: Collection<Group> = db.collection("groups");
    if let Ok(Some(_)) = group_coll
        .find_one(doc! {"uuid": &uuid, "created_by": &req_uuid}, None)
        .await
    {
        group_coll
            .update_one(
                doc! {"uuid": &uuid, "created_by": &req_uuid},
                doc! {"$set": {"name": name, "subjects": bson::to_bson(&subjects).unwrap(), "description": description}},
                None,
            )
            .await
            .unwrap();
        json!({ "response" : "OK"})
    } else {
        json!({ "response" : "ERROR", "text": "Group does not exist or was not created by user with given key."})
    }
}
