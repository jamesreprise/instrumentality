//! Route for adding data to Instrumentality.
//!
//! The /add route is implemented here.
//!
//! See endpoint documentation at <https://docs.berserksystems.com/endpoints/add/>.
//!
//! See [`Data`] for examples of valid data objects.

use crate::config::IConfig;
use crate::data::{Data, Datas};
use crate::database::DBHandle;
use crate::key::Key;
use crate::response::{Error, Ok};
use crate::user::User;

use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::Collection;

pub async fn add(
    key: Key,
    Json(data): Json<Datas>,
    db: DBHandle,
    config: IConfig,
) -> impl IntoResponse {
    let data = data
        .verify(&config.content_types, &config.presence_types)
        .tag(User::with_key(&key.key, &db).await.unwrap().uuid)
        .process_queue(&db)
        .await;
    let data_coll: Collection<Data> = db.collection("data");
    // We need to merge existing Data::Content here.
    // If it already exists, check if the new data is a superset of the
    // existing content and replace it. If not, merge the data so that it
    // is a union of the two. We should only ever be turning Option::None
    // into Option::Some(T) in this case.
    // Additionally, deleted can become true.
    if !data.data.is_empty() {
        data_coll.insert_many(data.data, None).await.unwrap();
        Ok((StatusCode::OK, Json(Ok::new())))
    } else {
        Err((StatusCode::NOT_ACCEPTABLE, Json(Error::new("No valid data was submitted. Ensure the given platforms and content/presence types are supported by this server. Ensure all data was correctly labeled for queue jobs."))))
    }
}
