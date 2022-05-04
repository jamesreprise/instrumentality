//! Route for deleting subjects and groups.
//!
//! The /delete route is implemented here.
//!
//! Only [`Subject`]s and [`Group`]s can be deleted as they exist solely within
//! Instrumentality. We cannot delete profiles (or data about them) as they
//! exist elsewhere.
//!
//! Only the user who created a Subject or Group can delete them, and do so
//! using their UUID.
//!
//! A valid /delete request takes the form:
//! ```json
//! {
//!     "uuid": "<UUIDv4>"
//! }
//! ```
//! Which would yield the response:
//! ```json
//! {
//!     "response": "OK"
//! }

use crate::group::Group;
use crate::key::Key;
use crate::mdb::DBHandle;
use crate::response::Error;
use crate::response::Ok;
use crate::routes::queue;
use crate::subject::*;
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteData {
    uuid: String,
}

// This is ugly. Can probably do better than an if-else.
pub async fn delete(data: Json<DeleteData>, db: DBHandle, key: Key) -> impl IntoResponse {
    let data: DeleteData = data.0;
    // UUID of the requester.
    let req_uuid = User::user_with_key(&key.key, &db).await.unwrap().uuid;
    let subj_coll: Collection<Subject> = db.collection("subjects");
    if let Ok(Some(subject)) = subj_coll
        .find_one(doc! {"uuid": &data.uuid, "created_by": &req_uuid}, None)
        .await
    {
        let group_coll: Collection<Group> = db.collection("groups");
        let result = group_coll
            .update_many(
                doc! {"subjects": &data.uuid},
                doc! {"$pull": {"subjects": &data.uuid}},
                None,
            )
            .await;

        if result.is_ok() {
            subj_coll
                .delete_one(doc! {"uuid": &data.uuid, "created_by": &req_uuid}, None)
                .await
                .unwrap();

            for platform in subject.profiles.keys() {
                for id in subject.profiles.get(platform).unwrap() {
                    queue::remove_queue_item(id, platform, &db).await;
                }
            }

            Ok((StatusCode::OK, Json(Ok::new())))
        } else {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::new("Internal server error.")),
            ))
        }
    } else {
        let group_coll: Collection<Subject> = db.collection("groups");
        if let Ok(Some(_)) = group_coll
            .find_one(doc! {"uuid": &data.uuid, "created_by": &req_uuid}, None)
            .await
        {
            group_coll
                .delete_one(doc! {"uuid": &data.uuid, "created_by": &req_uuid}, None)
                .await
                .unwrap();
            Ok((StatusCode::OK, Json(Ok::new())))
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                Json(Error::new("No such group or subject exists or it was not created by the user with the given key.")),
            ))
        }
    }
}
