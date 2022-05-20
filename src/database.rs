//! Database functions and implementations for Instrumentality.

use crate::config::IConfig;
use crate::data::Data;
use crate::group::Group;
use crate::subject::Subject;
use crate::user::User;

use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::response::Response;
use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::results::CreateIndexResult;
use mongodb::Cursor;
use mongodb::{bson::doc, Client, Collection, Database, IndexModel};
use std::time::Duration;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct DBPool {
    client: Client,
    database: String,
}

impl DBPool {
    pub fn handle(&self) -> DBHandle {
        DBHandle {
            db: self.client.database(&self.database),
        }
    }
}

pub struct DBHandle {
    db: Database,
}

impl DBHandle {
    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        self.db.collection::<T>(name)
    }

    pub async fn drop(&self) -> Result<(), mongodb::error::Error> {
        self.db.drop(None).await
    }
}

pub async fn open(config: &IConfig) -> Result<DBPool, Box<dyn std::error::Error>> {
    let user = &config.mongodb.user;
    let password = &config.mongodb.password;
    let hosts = &config.mongodb.hosts;
    let port = &config.mongodb.port;
    let database = &config.mongodb.database;
    let mut mongo_options =
        ClientOptions::parse(format!("mongodb://{user}:{password}@{hosts}:{port}")).await?;
    mongo_options.connect_timeout = Some(Duration::new(1, 0));
    mongo_options.heartbeat_freq = Some(Duration::new(1, 0));
    mongo_options.server_selection_timeout = Some(Duration::new(1, 0));
    let mongo_client = Client::with_options(mongo_options).unwrap();
    let database = mongo_client.database(database);

    // It is only at this point that MongoDB actually makes a connection.
    database
        .run_command(doc! {"ping" : 1_u32}, None)
        .await
        .expect("Couldn't connect to MongoDB");

    let user_count: u64 = database
        .collection::<User>("users")
        .count_documents(None, None)
        .await
        .unwrap();
    if user_count == 0 {
        tracing::info!("Creating root account...");
        let root_user = create_root_account(&database).await.unwrap();
        tracing::info!("\n{:#?}", root_user);
        tracing::info!("Creating indexes...");
        create_indexes(&database).await;
    }

    Ok(DBPool {
        client: mongo_client,
        database: config.mongodb.database.to_string(),
    })
}

async fn create_root_account(database: &Database) -> Result<User, Box<dyn std::error::Error>> {
    let users_coll: Collection<User> = database.collection("users");
    let user = User::new("root");
    users_coll.insert_one(&user, None).await.unwrap();
    Ok(user)
}

async fn create_indexes(database: &Database) {
    unique_content_index(database).await.unwrap();
    unique_subject_name_index(database).await.unwrap();
    users_key_index(database).await.unwrap();
    users_user_index(database).await.unwrap();
    users_key_banned_index(database).await.unwrap();
    queue_platform_and_id_index(database).await.unwrap();
    subjects_uuid_index(database).await.unwrap();
    referrals_code_used_index(database).await.unwrap();
    queue_id_index(database).await.unwrap();
    data_id_platform_index(database).await.unwrap();
}

async fn unique_subject_name_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Unique Subject Name"))
        .unique(true)
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"created_by" : 1_u32, "name": 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Subject>("subjects")
        .create_index(idx_model, None)
        .await
}

async fn unique_content_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Unique Content ID"))
        .unique(true)
        .sparse(true)
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"content_id" : 1_u32, "platform": 1_u32, "content_type" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("data")
        .create_index(idx_model, None)
        .await
}

async fn users_key_index(database: &Database) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Users Key Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"key" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("users")
        .create_index(idx_model, None)
        .await
}

async fn users_key_banned_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Users Key & Banned Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"key" : 1_u32, "banned" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("users")
        .create_index(idx_model, None)
        .await
}

async fn users_user_index(database: &Database) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Users User Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"user" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("users")
        .create_index(idx_model, None)
        .await
}

async fn queue_platform_and_id_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Queue Platform and Platform ID Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"platform" : 1_u32, "platform_id" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("queue")
        .create_index(idx_model, None)
        .await
}

async fn subjects_uuid_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Subjects UUID Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"uuid" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("subjects")
        .create_index(idx_model, None)
        .await
}

async fn referrals_code_used_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Referrals Code Used Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"code" : 1_u32, "used" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("referrals")
        .create_index(idx_model, None)
        .await
}

async fn queue_id_index(database: &Database) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Queue ID Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"queue_id" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("queue")
        .create_index(idx_model, None)
        .await
}

async fn data_id_platform_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Data Platform ID & Platform Index"))
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! {"id" : 1_u32, "platform" : 1_u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("data")
        .create_index(idx_model, None)
        .await
}

pub async fn drop_database(database: &DBHandle) {
    database.drop().await.unwrap();
}

#[async_trait]
impl<B: Send> FromRequest<B> for DBHandle {
    type Rejection = Response;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let db_pool = request.extensions().get::<DBPool>().unwrap();

        let db = db_pool.handle();

        Ok(db)
    }
}

pub async fn user_with_key(key: &str, db: &DBHandle) -> Option<User> {
    let users_coll: Collection<User> = db.collection("users");
    let result = users_coll.find_one(doc! {"key": key}, None).await.unwrap();
    result
}

pub async fn user_subjects(user: &User, db: &DBHandle) -> Option<Vec<Subject>> {
    let subj_coll: Collection<Subject> = db.collection("subjects");
    let cursor: Cursor<Subject> = subj_coll
        .find(doc! {"created_by": &user.uuid}, None)
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

pub async fn user_groups(user: &User, db: &DBHandle) -> Option<Vec<Group>> {
    let group_coll: Collection<Group> = db.collection("groups");
    let cursor: Cursor<Group> = group_coll
        .find(doc! {"created_by": &user.uuid}, None)
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
