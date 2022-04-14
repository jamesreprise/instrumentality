//! Route for the queue.
//!
//! The queue is a looping structure containing all the profiles currently
//! being tracked by Instrumentality. Profiles are only being tracked if they
//! exist under a subject.
//!
//! # Locking
//! An efficient queue structure that has jobs that takes time needs a system
//! of locks in order to ensure that data providers are not working over one
//! another. This massively increases the system throughput given that each
//! fetch has an opportunity cost.
//!
//! # Incentives
//! Doing jobs in the queue should be preferable to simply posting whatever
//! data the provider cares to. Ideally there is a leaderboard that awards
//! points based on work done. This would have to reset monthly to allow for
//! fair competition to new providers.
//!
//! # Username changes
//! A fundamental problem with a queue is that we store all our data in terms
//! of platform-unique user IDs rather than by username. This is because
//! platforms generally allow for usernames to be changed, and we must allow
//! for this locally. So when we want to refresh the data we have on a subject,
//! we need to be able to turn an ID into a username that may have changed.
//! This is assuming that the platform doesn't allow outside lookups by ID.
//!
//! The most simple way of doing this is taking the ID from the subject, going
//! to metadata under our data collection in MongoDB and searching for that
//! user's most recent 'username' for that platform. Then we trust that a data
//! provider is going to be able to turn a (platform, username) pair into fresh
//! content/presence/meta data. This will succeed _most_ of the time.
//!
//! In the event that the username has changed, the above method will fail upon
//! finding that the profile is empty or has been replaced by a user with a
//! different ID. Given that there is no way to turn IDs to usernames, we can
//! only advise data providers to use older data from that platform user such
//! as searching previous content posts for users with the same ID. However,
//! this will be heavily platform specific and falls outside the scope of
//! Instrumentality.
//!
//! # Round robin vs. Alternatives
//! A naive queue implementation would be to take every platform user and cycle
//! them, putting most recently fetched data at the bottom of the queue.
//!
//! However, this might not be ideal. Intuitively, a user that posts data is
//! more likely to post data again soon than one that hasn't posted recently.
//! If we know that a user has posted data recently, we want to prioritise
//! fetching them again soon in order to catch more data about them in case
//! of deletion. This is assuming some level of opportunity cost with each
//! fetch.
//!
//! We still want to guarantee some level of coverage to all profiles and don't
//! wish to tune this to be so aggressive that profiles that happen to not
//! have any recent activity become stuck at the back of the queue.
//!
//! One method of implementing this is a hot and cold queue. Naturally, the
//! queue will still be presented to data providers as a single queue, but
//! Instrumentality will interleave the hot queue in at the top of the global
//! queue in order to ensure new hot profiles are still being detected.
//!
//! Additionally, profiles under a single subject become hot by association.

use crate::data::Data;
use crate::key::Key;

use chrono::offset::TimeZone;
use chrono::{DateTime, Duration, Utc};
use mongodb::bson::Bson;
use mongodb::options::{FindOneAndUpdateOptions, FindOneOptions};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::Collection;
use mongodb::{bson::doc, Database};
use rocket::serde::json::Value;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct InternalQueueItem {
    uuid: String, // Queue ID.
    platform_id: String,
    platform: String,
    last_processed: DateTime<Utc>,
    lock_holder: Option<String>, // None means not locked.
    lock_acquired_at: Option<DateTime<Utc>>,
}

impl InternalQueueItem {
    fn new(platform_id: String, platform: String) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            platform_id,
            platform,
            last_processed: Utc.ymd(1970, 1, 1).and_hms(0, 1, 1),
            lock_holder: None,
            lock_acquired_at: None,
        }
    }
}

#[get("/queue?<platforms>")]
pub async fn queue(platforms: Vec<String>, db: &State<Database>, key: Key) -> Value {
    // This is not optimal for performance. Should be running as a scheduled task in a thread.
    clear_old_locks(&db).await;

    let filter_builder =
        FindOneAndUpdateOptions::builder().sort(doc! {"last_processed": -1 as i32});

    let filter = filter_builder.build();

    let q_coll: Collection<InternalQueueItem> = db.collection("queue");
    let result = q_coll
        .find_one_and_update(
            doc! { "lock_holder": Bson::Null, "platform": {"$in": &platforms} },
            doc! { "$set": {"lock_holder": &key.key, "lock_acquired_at": Utc::now().to_string()}},
            filter,
        )
        .await
        .unwrap()
        .unwrap();

    let filter_builder = FindOneOptions::builder().projection(doc! {"username": 1 as i32});

    let filter = filter_builder.build();

    #[derive(Debug, Serialize, Deserialize)]
    struct Username {
        username: String,
    }

    let data_coll: Collection<Data> = db.collection("data");
    let username = data_coll
        .clone_with_type::<Username>()
        .find_one(
            doc! {"id": &result.platform_id, "platform": &result.platform},
            filter,
        )
        .await
        .unwrap()
        .unwrap();

    json!({ "uuid": &result.uuid, "username": &username.username, "platform": &result.platform })
}

pub async fn process(
    queue_id: &Option<String>,
    id: &String,
    platform: &String,
    added_by: &Option<String>,
    db: &State<Database>,
) -> bool {
    let added_by = added_by.as_ref().unwrap();
    if let Some(queue_id) = queue_id {
        let q_coll: Collection<InternalQueueItem> = db.collection("queue");
        let update_result = q_coll
            .update_one(
                doc! { "queue_id" : queue_id, "platform_id": id, "platform": platform, "lock_holder": added_by },
                doc! { "$set": { "lock_holder": Bson::Null, "last_processed": Utc::now().to_string() } },
                None,
            )
            .await
            .unwrap();
        update_result.modified_count == 1
    } else {
        false
    }
}

pub async fn add_queue_item(
    platform_id: &String,
    platform: &String,
    db: &State<Database>,
) -> Result<InsertOneResult, mongodb::error::Error> {
    let q_coll: Collection<InternalQueueItem> = db.collection("queue");
    let queue_item: InternalQueueItem =
        InternalQueueItem::new(platform_id.clone(), platform.clone());
    Ok(q_coll.insert_one(queue_item, None).await.unwrap())
}

pub async fn remove_queue_item(
    platform_id: &String,
    platform: &String,
    db: &State<Database>,
) -> Result<DeleteResult, mongodb::error::Error> {
    let q_coll: Collection<InternalQueueItem> = db.collection("queue");
    Ok(q_coll
        .delete_one(
            doc! { "platform_id": platform_id, "platform": platform },
            None,
        )
        .await
        .unwrap())
}

pub async fn clear_old_locks(db: &State<Database>) {
    let q_coll: Collection<InternalQueueItem> = db.collection("queue");
    let thirty_seconds_ago: DateTime<Utc> = Utc::now() - Duration::seconds(30);
    q_coll
        .update_many(
            doc! { "lock_acquired_at": {"$lt": thirty_seconds_ago.to_string() } },
            doc! { "$set": {"lock_acquired_at": Bson::Null, "lock_holder": Bson::Null }},
            None,
        )
        .await
        .unwrap();
}
