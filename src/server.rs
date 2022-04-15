use rocket::*;

use crate::routes::add::*;
use crate::routes::catchers::*;
use crate::routes::create::*;
use crate::routes::delete::*;
use crate::routes::frontpage::*;
use crate::routes::invite::*;
use crate::routes::login::*;
use crate::routes::queue::*;
use crate::routes::register::*;
use crate::routes::types::*;
use crate::routes::update::*;
use crate::routes::view::*;

use crate::config;
use crate::mdb;

use rocket::fairing::AdHoc;
use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use rocket::fs::{relative, FileServer};

pub async fn build_rocket() -> rocket::Rocket<Ignite> {
    let figment =
        Figment::from(rocket::Config::default()).merge(Toml::file("Rocket.toml").nested());
    let iconfig = config::open().unwrap();
    let database = mdb::open(&iconfig).await.unwrap();
    rocket::custom(figment)
        .mount(
            "/",
            routes![
                register, invite, login, types, add, view, queue, create, delete, update, frontpage
            ],
        )
        .mount("/", FileServer::from(relative!("files")))
        .register("/", catchers![default_err])
        .attach(AdHoc::on_ignite("Config", |rocket| async move {
            rocket.manage(iconfig)
        }))
        .attach(AdHoc::on_ignite("MongoDB", |rocket| async move {
            rocket.manage(database)
        }))
        .ignite()
        .await
        .unwrap()
}
