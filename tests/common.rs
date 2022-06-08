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
use instrumentality::config;
use instrumentality::config::IConfig;
use instrumentality::database;
use instrumentality::response::LoginResponse;
use instrumentality::server;
use instrumentality::user::User;

use axum::Router;
use hyper::{Body, Request, StatusCode};
use tower::Service;
use uuid::Uuid;

pub const TEST_ENVIRONMENT_CONFIG: &str = "InstrumentalityTest.toml";

pub struct Environment {
    pub app: Router,
    pub user: User,
    pub config: IConfig,
}

impl Environment {
    pub async fn new(config_path: &str) -> Self {
        let mut config = config::open(config_path).unwrap();
        let test_db_id = Uuid::new_v4().to_string();
        config.mongodb.database = test_db_id.clone();
        let app = Self::setup_server(&config).await;

        let user = Self::inject_test_account(&config).await;

        Self { app, user, config }
    }

    pub async fn cleanup(self) {
        let database = database::open(&self.config).await.unwrap();
        database::drop_database(&database.handle()).await;
    }

    // This is only used in tests, so it flags as dead code.
    #[allow(dead_code)]
    pub async fn login(&mut self) -> LoginResponse {
        let res = self
            .app
            .call(
                Request::builder()
                    .method("GET")
                    .header("X-API-KEY", &self.user.key)
                    .uri("/login")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let lr: LoginResponse = serde_json::from_slice(&body).unwrap();

        lr
    }

    // Provides a client to process requests to Instrumentality without going over
    // a network.
    async fn setup_server(iconfig: &IConfig) -> Router {
        let (app, _, _) = server::build_server(iconfig).await;

        app
    }

    async fn inject_test_account(iconfig: &IConfig) -> User {
        let database = database::open(&iconfig).await.unwrap();

        let user = User::new("test");
        let _user_coll = database
            .handle()
            .collection::<User>("users")
            .insert_one(&user, None)
            .await
            .unwrap();
        user
    }
}
