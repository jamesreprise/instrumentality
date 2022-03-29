use crate::config::IConfig;
use crate::data::Data;
use crate::subject::Subject;
use crate::user::User;

use mongodb::options::{ClientOptions, IndexOptions};
use mongodb::results::CreateIndexResult;
use mongodb::{bson::doc, Client, Database};
use mongodb::{Collection, IndexModel};
use std::time::Duration;

pub async fn open(config: &IConfig) -> Result<Database, Box<dyn std::error::Error>> {
    // let user = config.mdb_config.user;
    // let password = config.mdb_config.password;
    let hosts = &config.mongodb.hosts;
    let port = &config.mongodb.port;
    let database = &config.mongodb.database;
    let mut mongo_options = ClientOptions::parse(format!("mongodb://{hosts}:{port}")).await?;
    mongo_options.connect_timeout = Some(Duration::new(1, 0));
    mongo_options.heartbeat_freq = Some(Duration::new(3, 0));
    mongo_options.server_selection_timeout = Some(Duration::new(5, 0));
    let mongo_client = Client::with_options(mongo_options).unwrap();
    let database = mongo_client.database(&database);

    // It is only at this point that MongoDB actually makes a connection.
    database
        .run_command(doc! { "ping" : 1}, None)
        .await
        .expect("Couldn't connect to MongoDB");

    let user_count: u64 = database
        .collection::<User>("users")
        .count_documents(None, None)
        .await
        .unwrap();
    if user_count == 0 {
        create_root_account(&database).await.unwrap();
        create_indexes(&database).await;
    }

    Ok(database)
}

async fn create_root_account(database: &Database) -> Result<User, Box<dyn std::error::Error>> {
    let users_coll: Collection<User> = database.collection("users");
    let user = User::new("root");
    users_coll.insert_one(&user, None).await.unwrap();
    Ok(user)
}

async fn create_indexes(database: &Database) -> () {
    unique_content_index(database).await.unwrap();
    unique_subject_name_index(database).await.unwrap();
}

async fn unique_subject_name_index(
    database: &Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let idx_options = IndexOptions::builder()
        .name(String::from("Unique Subject Name"))
        .unique(true)
        .sparse(true)
        .build();

    let idx_model = IndexModel::builder()
        .keys(doc! { "created_by" : 1, "name": 1})
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
        .keys(doc! { "content_id" : 1 as u32, "platform": 1 as u32, "content_type" : 1 as u32})
        .options(idx_options)
        .build();

    database
        .collection::<Data>("data")
        .create_index(idx_model, None)
        .await
}
