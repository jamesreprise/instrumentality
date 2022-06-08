// //! Ensure you have read the doc comments in common.rs if you are having
// //! difficulty getting tests to work.

// mod common;
// use common::Environment;
// use common::TEST_ENVIRONMENT_CONFIG;

// use axum::http::StatusCode;
// use hyper::Body;
// use hyper::Request;
// use tower::Service;

// /// test_authorised_add tests:
// /// - Authentication of the test user works as expected.
// /// - Upon receiving valid data the add route returns:
// ///     - an OK.
// #[tokio::test]
// async fn test_authorised_add() {
//     use instrumentality::data::Data;
//     use instrumentality::response::Ok;

//     let mut env: Environment = Environment::new(TEST_ENVIRONMENT_CONFIG).await;

//     let data = Data::Content {};

//     let res = env
//         .app
//         .call(
//             Request::builder()
//                 .method("POST")
//                 .header("X-API-KEY", &env.user.key)
//                 .header(
//                     axum::http::header::CONTENT_TYPE,
//                     mime::APPLICATION_JSON.as_ref(),
//                 )
//                 .uri("/add")
//                 .body(Body::from(serde_json::to_vec(&data).unwrap()))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();

//     assert_eq!(res.status(), StatusCode::OK);

//     let body = hyper::body::to_bytes(res.into_body()).await.unwrap();
//     let ok: Ok = serde_json::from_slice(&body).unwrap();

//     assert_eq!(ok.response, "OK".to_string());
//     env.cleanup().await;
// }
