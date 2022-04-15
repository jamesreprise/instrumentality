mod common;

use instrumentality::rocket::async_test;
use instrumentality::rocket::http::Status;
use rocket::local::asynchronous::Client;

#[async_test]
async fn test_alive() {
    let client: Client = common::setup_client().await;
    let res = client.get("/").dispatch().await;
    assert_eq!(res.status(), Status::Ok)
}
