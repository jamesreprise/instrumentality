//! This is the API documentation for the Instrumentality server. It contains technical
//! details about the internal operation of instrumentality, and is solely aimed at
//! developers looking to understand or extend the servers capabilities.
//!
//! You can find documentation on installation and running the server at:
//! - <https://github.com/berserksystems/instrumentality.git>
//!
//! Instrumentality makes heavy use of [Rocket] and [MongoDB]. MongoDB in particular is
//! probably not the correct choice for this system at scale, and a re-write using
//! Postgres would be more performant but less flexible. Rocket is also potentially not
//! the perfect library for [performance] at scale. However, it has made writing this
//! prototype much easier.
//!
//! No effort has been made to verify the above two claims.
//!
//! [Rocket]: https://rocket.rs/
//! [MongoDB]: https://www.mongodb.com/
//! [performance]: https://matej.laitl.cz/bench-actix-rocket/

#[macro_use]
extern crate rocket;

pub mod config;
pub mod data;
pub mod group;
pub mod key;
pub mod mdb;
pub mod routes;
pub mod server;
pub mod subject;
pub mod user;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    server::build_rocket("Instrumentality.toml")
        .await
        .launch()
        .await
}
