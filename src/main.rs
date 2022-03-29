//! # Instrumentality - Documentation
//!
//! This is the API documentation for the Instrumentality server. It contains technical
//! details about the internal operation of instrumentality, and is solely aimed at
//! developers looking to understand or extend the servers capabilities.
//!
//! You can find documentation on installation and running the server at:
//! - <https://instrumentality.berserksystems.com/docs/install>
//! - <https://instrumentality.berserksystems.com/docs/run>
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

use crate::routes::add::*;
use crate::routes::catchers::*;
use crate::routes::create::*;
use crate::routes::delete::*;
use crate::routes::invite::*;
use crate::routes::login::*;
use crate::routes::register::*;
use crate::routes::types::*;
use crate::routes::update::*;
use crate::routes::view::*;

use rocket::fairing::AdHoc;
use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use rocket::fs::{relative, FileServer};

mod config;
mod data;
mod group;
mod key;
mod mdb;
mod profile;
mod routes;
mod subject;
mod user;

#[launch]
async fn rocket() -> _ {
    let figment =
        Figment::from(rocket::Config::default()).merge(Toml::file("Rocket.toml").nested());
    let iconfig = config::open().unwrap();
    let database = mdb::open(&iconfig).await.unwrap();
    rocket::custom(figment)
        .mount("/", routes![add])
        .mount("/", routes![register])
        .mount("/", routes![invite])
        .mount("/", routes![types])
        .mount("/", routes![create])
        .mount("/", routes![delete])
        .mount("/", routes![view])
        .mount("/", routes![update])
        .mount("/", routes![login])
        .mount("/", FileServer::from(relative!("files")))
        .register("/", catchers![default_err])
        .attach(AdHoc::on_ignite("Config", |rocket| async move {
            rocket.manage(iconfig)
        }))
        .attach(AdHoc::on_ignite("MongoDB", |rocket| async move {
            rocket.manage(database)
        }))
}
