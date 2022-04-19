//! Ensure you have read the doc comments in common.rs if you are having
//! difficulty getting tests to work.

mod common;

use common::prepare_environment;
use instrumentality::rocket::async_test;
use instrumentality::rocket::http::Status;

use rocket::http::Header;
// use rocket::local::asynchronous::Client;

static TEST_ENVIRONMENT_CONFIG: &str = "InstrumentalityTest.toml";

/// test_alive tests:
/// - Instrumentality serves an OK response to a request to the root.
#[async_test]
async fn test_alive() {
    let (client, _) = prepare_environment(TEST_ENVIRONMENT_CONFIG).await;
    let res = client.get("/").dispatch().await;

    assert_eq!(res.status(), Status::Ok)
}

/// test_catcher_404 tests:
/// - Instrumentality serves a NOT FOUND error to a request to an invalid
///   route i.e. (/404)
#[async_test]
async fn test_catcher_404() {
    use instrumentality::routes::catchers::ErrorResponse;

    let (client, _) = prepare_environment(TEST_ENVIRONMENT_CONFIG).await;
    let res = client.get("/404").dispatch().await;
    assert_eq!(res.status(), Status::NotFound);

    let er = res.into_json::<ErrorResponse>().await.unwrap();

    assert_eq!(er.response, "ERROR".to_string());
    assert_eq!(er.error_code, 404);
}

/// test_no_key_login tests:
/// - Authentication without a X-API-KEY header returns not authorised.
#[async_test]
async fn test_no_key_login() {
    use instrumentality::routes::catchers::ErrorResponse;

    let (client, _) = prepare_environment(TEST_ENVIRONMENT_CONFIG).await;

    let res = client.get("/login").dispatch().await;
    let er: ErrorResponse = res.into_json::<ErrorResponse>().await.unwrap();

    assert_eq!(er.response, "ERROR".to_string());
    assert_eq!(er.error_code, 401);
}

/// test_bad_key_login tests:
/// - Authentication without a X-API-KEY header returns not authorised.
#[async_test]
async fn test_bad_key_login() {
    use instrumentality::routes::catchers::ErrorResponse;

    let (client, _) = prepare_environment(TEST_ENVIRONMENT_CONFIG).await;

    let auth_header = Header::new("X-API-KEY", "INVALIDAPIKEY");

    let res = client.get("/login").header(auth_header).dispatch().await;
    let er: ErrorResponse = res.into_json::<ErrorResponse>().await.unwrap();

    assert_eq!(er.response, "ERROR".to_string());
    assert_eq!(er.error_code, 401);
}

/// test_authorised_login tests:
/// - Authentication of the test user works as expected.
/// - Login route returns the correct information:
///     - an OK,
///     - the user info,
///     - empty subjects and groups array
#[async_test]
async fn test_authorised_login() {
    use instrumentality::routes::login::LoginResponse;

    let (client, user) = prepare_environment(TEST_ENVIRONMENT_CONFIG).await;

    let auth_header = Header::new("X-API-KEY", user.clone().key);

    let res = client.get("/login").header(auth_header).dispatch().await;
    let lr = res.into_json::<LoginResponse>().await.unwrap();

    assert_eq!(lr.response, "OK".to_string());
    assert_eq!(lr.user, user.clone());
    assert!(lr.subjects.is_empty());
    assert!(lr.groups.is_empty());
}
