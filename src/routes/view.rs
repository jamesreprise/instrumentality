//! Route for viewing data about a subject or group.
//!
//! The /view route is implemented here.
//!
//! View is a type of search on the data held in the system.
//! The most typical search will be looking up recent posts on all platforms
//! for a specific subject/group/profile.
//!
//! We can think of a search as a series of scope 'broadeners' and 'narrowers'.
//! Currently, every subject is a 'broadener', every other parameter is a 'narrower'.

use crate::data::*;
use crate::database::DBHandle;
use crate::key::Key;
use crate::response::{Error, ViewResponse};
use crate::subject::Subject;

use axum::{extract::Query, http::StatusCode, Json};
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::options::FindOptions;
use mongodb::Collection;
use serde::{Deserialize, Deserializer, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewData {
    subject_data: Vec<SubjectData>,
}

impl ViewData {
    fn new() -> Self {
        Self {
            subject_data: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubjectData {
    subject: Subject,
    platforms: Vec<PlatformData>,
}

impl SubjectData {
    fn new(subject: &Subject) -> Self {
        Self {
            subject: subject.clone(),
            platforms: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PlatformData {
    profiles: Vec<ProfileData>,
}

impl PlatformData {
    fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProfileData {
    meta: Option<Data>,
    content: Vec<Data>,
    presence: Vec<Data>,
}

impl ProfileData {
    fn new(meta: Option<Data>) -> Self {
        Self {
            meta,
            content: Vec::new(),
            presence: Vec::new(),
        }
    }
}

// https://github.com/tokio-rs/axum/issues/434#issuecomment-954924025
// No support for vec in query. Using workaround by jplatte.
#[derive(Deserialize)]
pub struct ViewQuery {
    #[serde(deserialize_with = "deserialize_array")]
    subjects: Vec<String>,
}

fn deserialize_array<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let nb = s
        .chars()
        .filter(|c| vec!['[', ']'].contains(c))
        .collect::<String>();
    let v = nb.split(",").map(|s| s.into()).collect::<Vec<String>>();

    Ok(v)
}

pub async fn view(
    view_query: Option<Query<ViewQuery>>,
    db: DBHandle,
    _key: Key,
) -> Result<(StatusCode, Json<ViewResponse>), (StatusCode, Json<Error>)> {
    if view_query.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(Error::new("You must provide a list of subjects.")),
        ));
    }

    let subjects = &view_query.unwrap().subjects;

    let data_coll: Collection<Data> = db.collection("data");
    let filter_builder = FindOptions::builder()
        .limit(100)
        .sort(doc! {"retrieved_at": -1_i32})
        .batch_size(100);
    let filter = filter_builder.build();

    let subj_coll: Collection<Subject> = db.collection("subjects");
    let doc: Document = doc! {"uuid": {"$in": &subjects}};
    let subj_cursor = subj_coll.find(doc, None).await.unwrap();
    let results: Vec<Result<Subject, mongodb::error::Error>> = subj_cursor.collect().await;
    let subjects: Vec<Subject> = results.into_iter().map(|d| d.unwrap()).collect();

    let mut view_data = ViewData::new();

    for s in subjects {
        let mut subject_data: SubjectData = SubjectData::new(&s);
        for platform_name in s.profiles.keys() {
            let mut platform_data = PlatformData::new();
            for platform_id in s.profiles.get(platform_name).unwrap() {
                let f = filter.clone();
                let meta_data = data_coll
                    .find_one(
                        doc! {"id": &platform_id, "platform": &platform_name, "profile_picture": {"$exists": true}},
                        None,
                    )
                    .await
                    .unwrap();
                let mut profile_data: ProfileData = ProfileData::new(meta_data);

                let presence_cursor = data_coll
                    .find(
                        doc! {"id": &platform_id, "platform": &platform_name, "presence_type": {"$exists": true}},
                        f.clone(),
                    )
                    .await
                    .unwrap();
                let presence_data: Vec<Result<Data, mongodb::error::Error>> =
                    presence_cursor.collect().await;
                profile_data.presence = presence_data.into_iter().map(|d| d.unwrap()).collect();

                let content_cursor = data_coll
                    .find(
                        doc! {"id": &platform_id, "platform": &platform_name, "content_type": {"$exists": true}},
                        f.clone(),
                    )
                    .await
                    .unwrap();
                let content_data: Vec<Result<Data, mongodb::error::Error>> =
                    content_cursor.collect().await;
                profile_data.content = content_data.into_iter().map(|d| d.unwrap()).collect();

                platform_data.profiles.push(profile_data);
            }
            subject_data.platforms.push(platform_data);
        }
        view_data.subject_data.push(subject_data);
    }

    Ok((StatusCode::OK, Json(ViewResponse::new(view_data))))
}
