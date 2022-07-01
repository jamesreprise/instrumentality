//! Ensure you have read the doc comments in common.rs if you are having
//! difficulty getting tests to work.

mod common;
use common::Environment;
use common::TEST_ENVIRONMENT_CONFIG;

use axum::http::StatusCode;
use hyper::Body;
use hyper::Request;
use tower::Service;

/// test_alive tests:
/// - Instrumentality serves an OK response to a request to the root.
#[tokio::test]
async fn test_alive() {
    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    env.cleanup().await;
}
