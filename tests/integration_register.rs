//! Ensure you have read the doc comments in common.rs if you are having
//! difficulty getting tests to work.

mod common;
use common::Environment;
use common::TEST_ENVIRONMENT_CONFIG;

use axum::http::StatusCode;
use hyper::Body;
use hyper::Request;
use tower::Service;

/// test_invite_then_register tests:
/// - Authentication of the test user works as expected.
/// - Invite route returns the correct information:
///     - an OK,
///     - an invite code with a length of 128 characters containing only numbers
///       0 through 9 and letters A through F.
/// - Register route called with returned invite code allows creation of new
///   user.
/// - Login route called with created user's key is OK and has correct name.
#[tokio::test]
async fn test_invite_then_register() {
    use instrumentality::response::InviteResponse;
    use instrumentality::response::LoginResponse;
    use instrumentality::response::RegisterResponse;
    use instrumentality::routes::register::RegisterRequest;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .header("X-API-KEY", &env.user.key)
                .uri("/invite")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let ir: InviteResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(ir.response, "OK".to_string());

    let re = regex::Regex::new(r"^([A-F0-9])*$").unwrap();
    assert!(ir.code.len() == 128);
    assert!(re.is_match(&ir.code));

    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(
                    serde_json::to_vec(&RegisterRequest {
                        code: ir.code.clone(),
                        name: "test_invite_then_register".to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let rr: RegisterResponse = serde_json::from_slice(&body).unwrap();

    assert_eq!(rr.response, "OK".to_string());
    assert_eq!(rr.user.name, "test_invite_then_register".to_string());

    let res = env
        .app
        .call(
            Request::builder()
                .method("GET")
                .header("X-API-KEY", &rr.user.key)
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
    assert_eq!(lr.user.name, "test_invite_then_register");
    assert!(lr.subjects.is_empty());
    assert!(lr.groups.is_empty());

    env.cleanup().await;
}

/// test_invite_then_register tests:
/// - Authentication of the test user works as expected.
/// - Invite route returns the correct information:
///     - an OK,
///     - an invite code with a length of 128 characters containing only numbers
///       0 through 9 and letters A through F.
/// - Register route called with returned invite code allows creation of new
///   user.
/// - Login route called with created user's key is OK and has correct name.
#[tokio::test]
async fn test_register_bad_code() {
    use instrumentality::response::Error;
    use instrumentality::routes::register::RegisterRequest;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(
                    serde_json::to_vec(&RegisterRequest {
                        code: "MAKINGITUP".to_string(),
                        name: "test_invite_then_register".to_string(),
                    })
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let e: Error = serde_json::from_slice(&body).unwrap();

    assert_eq!(e.response, "ERROR".to_string());
}
