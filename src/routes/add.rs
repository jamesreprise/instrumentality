//! Route for adding data to Instrumentality.
//!
//! The /add route is implemented here.
//!
//! All data to be added must be formatted as below.
//! ```json
//! {"data" : [..]}
//! ```
//!
//! See [`Data`] for examples of valid data objects.

use crate::config::IConfig;
use crate::data::*;
use crate::key::Key;
use crate::user::User;

use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::serde::json::{Json, Value};
use rocket::State;
use serde_json::json;

#[post("/add", format = "json", data = "<data>")]
pub async fn add(
    key: Key,
    data: Json<Datas>,
    db: &State<Database>,
    config: &State<IConfig>,
) -> Value {
    let data = data
        .into_inner()
        .verify(config)
        .tag(User::user_with_key(&key.key, db).await.unwrap().uuid)
        .process_queue(db)
        .await;
    let data_coll: Collection<Data> = db.collection("data");
    // We need to merge existing Data::Content here.
    // If it already exists, check if the new data is a superset of the
    // existing content and replace it. If not, merge the data so that it
    // is a union of the two. We should only ever be turning Option::None
    // into Option::Some(T) in this case.
    // Additionally, deleted can become true.
    if data.data.len() != 0 {
        data_coll.insert_many(data.data, None).await.unwrap();
        json!({ "response" : "OK"})
    } else {
        json!({ "response" : "ERROR", "text": "No valid data was submitted."})
    }
}
