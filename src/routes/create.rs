//! Route for creating subjects and groups.
//!
//! The /create route is implemented here.
//!
//! ['Subject`]s and [`Group`]s are concepts that exist solely within
//! Instrumentality unlike [`Profile`]s of which the 'source of truth' exists
//! elsewhere.
//!
//! An example subject creation is given below:
//! ```json
//! {
//!     "name": "Subject1",
//!     "profiles":
//!         {
//!          "twitter": ["subject1", "subject1_private"],
//!          "instagram": ["subject1pics"]
//!         },
//!     "description": "Everyone's favourite subject."
//! }
//! ```
//! Which would yield the response:
//! ```json
//! {
//!     "response": "OK",
//!     "subject":
//!         {
//!          "uuid": "<UUIDv4>",
//!          "created_by": "<creator's UUIDv4>"
//!          "created_at": "<ISO8061 UTC TIME>"
//!          "name": "Subject1",
//!          "profiles":
//!             {
//!              "twitter": ["subject1", "subject1_private"],
//!              "instagram": ["subject1pics"]
//!             },
//!          "description": "Everyone's favourite subject."
//!         }
//! }
//! ```

use crate::database::DBHandle;
use crate::group::*;
use crate::key::Key;
use crate::response::{Error, Ok};
use crate::routes::queue;
use crate::subject::*;

use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CreateData {
    CreateSubject {
        name: String,
        profiles: HashMap<String, Vec<String>>,
        description: Option<String>,
    },
    CreateGroup {
        name: String,
        subjects: Vec<String>,
        description: Option<String>,
    },
}

pub async fn create(Json(data): Json<CreateData>, db: DBHandle, key: Key) -> impl IntoResponse {
    match data {
        CreateData::CreateSubject { .. } => {
            let subj_coll: Collection<Subject> = db.collection("subjects");
            if let Some(subject) = Subject::from_subject_create(data, &db, key).await {
                if subj_coll.insert_one(&subject, None).await.is_ok() {
                    for platform in subject.profiles.keys() {
                        for id in subject.profiles.get(platform).unwrap() {
                            queue::add_queue_item(id, platform, &db, false).await;
                        }
                    }
                    Ok((StatusCode::OK, Json(Ok::new())))
                } else {
                    Err((
                        StatusCode::CONFLICT,
                        Json(Error::new("Subject by that name already exists.")),
                    ))
                }
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(Error::new("Subject couldn't be created from data.")),
                ))
            }
        }
        CreateData::CreateGroup { .. } => {
            let group_coll: Collection<Group> = db.collection("groups");
            if let Some(group) = Group::from_group_create(data, &db, key).await {
                for s in &group.subjects {
                    let subj_coll: Collection<Subject> = db.collection("subjects");
                    let subject = subj_coll.find_one(doc! {"uuid": s}, None).await.unwrap();
                    if subject.is_none() {
                        return Err((
                            StatusCode::CONFLICT,
                            Json(Error::new("One or more of the subjects does not exist.")),
                        ));
                    }
                }
                if group_coll.insert_one(&group, None).await.is_ok() {
                    Ok((StatusCode::OK, Json(Ok::new())))
                } else {
                    Err((
                        StatusCode::CONFLICT,
                        Json(Error::new("Group by that name already exists.")),
                    ))
                }
            } else {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(Error::new("Group couldn't be created from data.")),
                ))
            }
        }
    }
}
