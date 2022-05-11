//! Ensure you have read the doc comments in common.rs if you are having
//! difficulty getting tests to work.
//!
//! TODO: Macro for env.cleanup().

mod common;
use common::Environment;

use axum::http::StatusCode;
use hyper::Body;
use hyper::Request;
use tower::Service;

static TEST_ENVIRONMENT_CONFIG: &str = "InstrumentalityTest.toml";

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

/// test_catcher_404 tests:
/// - Instrumentality serves a NOT FOUND error to a request to an invalid
///   route i.e. (/404)
#[tokio::test]
async fn test_catcher_404() {
    use instrumentality::response::Error;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .uri("/404")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let er: Error = serde_json::from_slice(&body).unwrap();

    assert_eq!(er.response, "ERROR".to_string());

    env.cleanup().await;
}

/// test_no_key_login tests:
/// - Authentication without a X-API-KEY header returns not authorised.
#[tokio::test]
async fn test_no_key_login() {
    use instrumentality::response::Error;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let er: Error = serde_json::from_slice(&body).unwrap();

    assert_eq!(er.response, "ERROR".to_string());

    env.cleanup().await;
}

/// test_bad_key_login tests:
/// - Authentication without a X-API-KEY header returns not authorised.
#[tokio::test]
async fn test_bad_key_login() {
    use instrumentality::response::Error;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .header("X-API-KEY", "INVALIDAPIKEY")
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let er: Error = serde_json::from_slice(&body).unwrap();

    assert_eq!(er.response, "ERROR".to_string());

    env.cleanup().await;
}

/// test_authorised_login tests:
/// - Authentication of the test user works as expected.
/// - Login route returns the correct information:
///     - an OK,
///     - the user info,
///     - empty subjects and groups array
#[tokio::test]
async fn test_authorised_login() {
    use instrumentality::routes::login::LoginResponse;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .header("X-API-KEY", &env.user.key)
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let lr: LoginResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(lr.response, "OK".to_string());
    assert_eq!(lr.user, env.user.clone());
    assert!(lr.subjects.is_empty());
    assert!(lr.groups.is_empty());

    env.cleanup().await;
}
