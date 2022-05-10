//! This is the API documentation for the Instrumentality server. It contains technical
//! details about the internal operation of instrumentality, and is solely aimed at
//! developers looking to understand or extend the servers capabilities.
//!
//! You can find documentation on installation and running the server at:
//! - <https://github.com/berserksystems/instrumentality.git>
//!
//! Instrumentality makes heavy use of [Axum] and [MongoDB]. MongoDB in particular is
//! probably not the correct choice for this system at scale, and a re-write using
//! Postgres would be more performant but less flexible.
//!
//! [MongoDB]: https://www.mongodb.com/
//! [Axum]: https://github.com/tokio-rs/axum/

pub mod config;
pub mod data;
pub mod database;
pub mod group;
pub mod key;
pub mod response;
pub mod routes;
pub mod server;
pub mod subject;
pub mod user;
