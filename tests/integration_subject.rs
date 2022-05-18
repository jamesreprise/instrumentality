//! Tests for creating, updating, deleting subjects.

mod common;
use common::Environment;
use common::TEST_ENVIRONMENT_CONFIG;

use axum::http::StatusCode;
use hyper::Body;
use hyper::Request;
use tower::Service;

/// test_subject_creation tests:
/// - Authentication of the test user works as expected.
/// - Subject is created upon post request.
/// - Subject can be seen via /login as provided with no changes.
#[tokio::test]
async fn test_subject_creation() {
    use instrumentality::response::LoginResponse;
    use instrumentality::response::Ok;
    use instrumentality::routes::create::CreateData;
    use std::collections::HashMap;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;
    let mut profiles: HashMap<String, Vec<String>> = HashMap::new();
    profiles.insert(
        "platform1".to_string(),
        vec!["user1".to_string(), "user1_priv".to_string()],
    );
    let new_subject = CreateData::CreateSubject {
        name: "test".to_string(),
        profiles,
        description: None,
    };
    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header("X-API-KEY", &env.user.key)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(serde_json::to_vec(&new_subject).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let or: Ok = serde_json::from_slice(&body).unwrap();

    assert_eq!(or.response, "OK".to_string());

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
    assert!(
        lr.subjects[0].profiles.get("platform1").unwrap()
            == &vec!["user1".to_string(), "user1_priv".to_string()]
    );
    assert!(lr.groups.is_empty());

    env.cleanup().await;
}

/// test_subject_bad_key_creation tests:
/// - /create requires authentication to create subject (or group).
#[tokio::test]
async fn test_subject_bad_key_creation() {
    use instrumentality::response::Error;
    use instrumentality::routes::create::CreateData;
    use std::collections::HashMap;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;
    let mut profiles: HashMap<String, Vec<String>> = HashMap::new();
    profiles.insert(
        "platform1".to_string(),
        vec!["user1".to_string(), "user1_priv".to_string()],
    );
    let new_subject = CreateData::CreateSubject {
        name: "test".to_string(),
        profiles,
        description: None,
    };
    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header("X-API-KEY", "MAKINGITUP")
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(serde_json::to_vec(&new_subject).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let e: Error = serde_json::from_slice(&body).unwrap();

    assert_eq!(e.response, "ERROR".to_string());

    env.cleanup().await;
}

/// test_subject_deletion tests:
/// - Authentication of the test user works as expected.
/// - Subject is created upon post request.
/// - Subject can be seen via /login as provided with no changes.
/// - Subject is removed upon deletion.
#[tokio::test]
async fn test_subject_deletion() {
    use instrumentality::response::LoginResponse;
    use instrumentality::response::Ok;
    use instrumentality::routes::create::CreateData;
    use instrumentality::routes::delete::DeleteData;
    use std::collections::HashMap;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;
    let mut profiles: HashMap<String, Vec<String>> = HashMap::new();
    profiles.insert(
        "platform1".to_string(),
        vec!["user1".to_string(), "user1_priv".to_string()],
    );
    let new_subject = CreateData::CreateSubject {
        name: "test".to_string(),
        profiles,
        description: None,
    };
    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header("X-API-KEY", &env.user.key)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(serde_json::to_vec(&new_subject).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let or: Ok = serde_json::from_slice(&body).unwrap();

    assert_eq!(or.response, "OK".to_string());

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
    let uuid = lr.subjects[0].uuid.clone();
    assert_eq!(lr.response, "OK".to_string());
    assert_eq!(lr.user, env.user.clone());
    assert!(
        lr.subjects[0].profiles.get("platform1").unwrap()
            == &vec!["user1".to_string(), "user1_priv".to_string()]
    );
    assert!(lr.groups.is_empty());

    let res = env
        .app
        .call(
            Request::builder()
                .method("DELETE")
                .uri("/delete")
                .header("X-API-KEY", &env.user.key)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(
                    serde_json::to_vec(&DeleteData { uuid }).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

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

/// test_subject_update tests:
/// - Authentication of the test user works as expected.
/// - Subject is created upon post request.
/// - Subject can be seen via /login as provided with no changes.
/// - Subject is updated correctly.
#[tokio::test]
async fn test_subject_update() {
    use instrumentality::response::LoginResponse;
    use instrumentality::response::Ok;
    use instrumentality::routes::create::CreateData;
    use instrumentality::routes::update::UpdateData;
    use std::collections::HashMap;

    let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;
    let mut profiles: HashMap<String, Vec<String>> = HashMap::new();
    profiles.insert(
        "platform1".to_string(),
        vec!["user1".to_string(), "user1_priv".to_string()],
    );
    let new_subject = CreateData::CreateSubject {
        name: "test".to_string(),
        profiles,
        description: None,
    };
    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/create")
                .header("X-API-KEY", &env.user.key)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(serde_json::to_vec(&new_subject).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
    let or: Ok = serde_json::from_slice(&body).unwrap();

    assert_eq!(or.response, "OK".to_string());

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
    let uuid = lr.subjects[0].uuid.clone();
    assert_eq!(lr.response, "OK".to_string());
    assert_eq!(lr.user, env.user.clone());
    assert!(
        lr.subjects[0].profiles.get("platform1").unwrap()
            == &vec!["user1".to_string(), "user1_priv".to_string()]
    );
    assert!(lr.groups.is_empty());

    let mut profiles: HashMap<String, Vec<String>> = HashMap::new();
    profiles.insert("platform1".to_string(), vec!["user1".to_string()]);
    profiles.insert(
        "platform2".to_string(),
        vec!["user1_on_platform2".to_string()],
    );
    let updated_subject = UpdateData::UpdateSubject {
        uuid,
        name: "test".to_string(),
        profiles,
        description: None,
    };

    let res = env
        .app
        .call(
            Request::builder()
                .method("POST")
                .uri("/update")
                .header("X-API-KEY", &env.user.key)
                .header(
                    axum::http::header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref(),
                )
                .body(Body::from(serde_json::to_vec(&updated_subject).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

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
    assert!(lr.subjects[0].profiles.get("platform1").unwrap() == &vec!["user1".to_string()]);
    assert!(
        lr.subjects[0].profiles.get("platform2").unwrap()
            == &vec!["user1_on_platform2".to_string()]
    );
    assert!(lr.groups.is_empty());

    env.cleanup().await;
}
