//! Ensure you have read the doc comments in common.rs if you are having
//! difficulty getting tests to work.

mod common;
use common::Environment;
use common::TEST_ENVIRONMENT_CONFIG;

use axum::http::StatusCode;
use hyper::Body;
use hyper::Request;
use tower::Service;

/// test_types tests:
/// - /types serves an OK response.
/// - the response corresponds with the given configuration.
#[tokio::test]
async fn test_types() {
    use instrumentality::response::TypesResponse;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .uri("/types")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let tr: TypesResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(tr.response, "OK");
    assert_eq!(tr.content_types, env.config.content_types);
    assert_eq!(tr.presence_types, env.config.presence_types);

    env.cleanup().await;
}
