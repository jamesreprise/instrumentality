//! Always use `prepare_environment` unless you are certain of what you are
//! doing. Using it twice will not create another user. Use the given user
//! to create an invite via /invite and register that user.
//!
//! `setup_client` starts instrumentality, which will connect to mongodb on
//! startup and check if the user collection is empty (shorthand for is this
//! a fresh database). If so, a root account is created as are some indexes.
//!
//! It is VITAL that you do not call `inject_test_account` before setting up
//! the client. If you do this, the indexes enforcing uniqueness on collections
//! will not be created and your test environment will yield subtly different
//! outcomes making debugging difficult.

use instrumentality::{config, mdb, rocket::local::asynchronous::Client, user::User};

pub async fn prepare_environment(config_path: &str) -> (Client, User) {
    drop_db(config_path).await;
    let client = setup_client(config_path).await;
    let user = inject_test_account(config_path).await;
    (client, user)
}

// Provides a rocket client to process requests to Instrumentality without
// going over a network.
pub async fn setup_client(config_path: &str) -> Client {
    Client::untracked(instrumentality::server::build_rocket(config_path).await)
        .await
        .unwrap()
}

pub async fn drop_db(config_path: &str) {
    let iconfig = config::open(config_path).unwrap();
    let database = mdb::open(&iconfig).await.unwrap();
    mdb::drop_database(&database).await;
}

pub async fn inject_test_account(config_path: &str) -> User {
    let iconfig = config::open(config_path).unwrap();
    let database = mdb::open(&iconfig).await.unwrap();

    let user = User::new("test");
    let _user_coll = database
        .collection::<User>("users")
        .insert_one(&user, None)
        .await
        .unwrap();
    user
}
