//! Route for creating subjects and groups.
//!
//! The /create route is implemented here.
//!
//! ['Subject`]s and [`Group`]s are concepts that exist solely within
//! Instrumentality unlike [`Profile`]s which the 'source of truth' exists
//! elsewhere.

use crate::group::*;
use crate::key::Key;
use crate::subject::*;

use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::serde::json::{Json, Value};
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CreateData {
    CreateSubject {
        name: String,
        profiles: HashMap<String, String>,
        description: Option<String>,
    },
    CreateGroup {
        name: String,
        subjects: Vec<String>,
        description: Option<String>,
    },
}

#[post("/create", format = "json", data = "<data>")]
pub async fn create(data: Json<CreateData>, db: &State<Database>, key: Key) -> Value {
    let data = data.into_inner();
    match data {
        CreateData::CreateSubject { .. } => create_subject(data, db, key).await,
        CreateData::CreateGroup { .. } => create_group(data, db, key).await,
    }
}

async fn create_subject(data: CreateData, db: &State<Database>, key: Key) -> Value {
    let data_coll: Collection<Subject> = db.collection("subjects");
    if let Some(subject) = Subject::from_subject_create(data, db, key).await {
        if let Ok(_) = data_coll.insert_one(&subject, None).await {
            json!({ "response" : "OK", "subject": &subject})
        } else {
            json!({ "response" : "ERROR", "text": "Subject by that name already exists."})
        }
    } else {
        json!({ "response" : "ERROR", "text": "Subject couldn't be created from data."})
    }
}

async fn create_group(data: CreateData, db: &State<Database>, key: Key) -> Value {
    let data_coll: Collection<Group> = db.collection("groups");
    if let Some(group) = Group::from_group_create(data, db, key).await {
        for s in &group.subjects {
            let subj_coll: Collection<Subject> = db.collection("subjects");
            let subject = subj_coll.find_one(doc! {"uuid": s}, None).await.unwrap();
            if let None = subject {
                return json!({ "response" : "ERROR", "text": "One or more of the subjects was not valid."});
            }
        }
        if let Ok(_) = data_coll.insert_one(&group, None).await {
            json!({ "response" : "OK", "group": &group})
        } else {
            json!({ "response" : "ERROR", "text": "Group by that name already exists."})
        }
    } else {
        json!({ "response" : "ERROR", "text": "Group couldn't be created from data."})
    }
}
