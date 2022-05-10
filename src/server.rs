//! Server functions for building Instrumentality.
//!
//! We build the tracing, service, router in this module.

use crate::config;
use crate::config::IConfig;
use crate::database;
use crate::database::DBPool;
use crate::response::Error;
use crate::routes::add::*;
use crate::routes::create::*;
use crate::routes::default::*;
use crate::routes::delete::*;
use crate::routes::frontpage::*;
use crate::routes::invite::*;
use crate::routes::login::*;
use crate::routes::queue::*;
use crate::routes::register::*;
use crate::routes::types::*;
use crate::routes::update::*;
use crate::routes::view::*;

use axum::http::header::{self, HeaderValue};
use axum::http::StatusCode;
use axum::middleware;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    handler::Handler,
    routing::{get, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::BoxError;
use tracing_subscriber::{prelude::*, EnvFilter};

pub async fn build_server(config: &str) -> (Router, RustlsConfig, SocketAddr) {
    build_tracing();

    let config = config::open(config).unwrap();
    tracing::info!("Config file loaded.");
    let db_pool = database::open(&config).await.unwrap();
    tracing::info!("Connected to MongoDB.");

    let app = build_app(config.clone(), db_pool);

    tracing::info!("Application built.");

    let tls_config = build_tls(&config.tls.cert, &config.tls.key).await;

    tracing::info!("TLS key & cert loaded.");
    let addr = build_address(&config.network.address, &config.network.port);

    (app, tls_config, addr)
}

fn build_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("INFO"))
        .init();
}

fn build_app(config: IConfig, db_pool: DBPool) -> Router {
    let service_builder = ServiceBuilder::new()
        .layer(middleware::from_fn(error_transformer))
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
            if error.is::<tower::timeout::error::Elapsed>() {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Error::new("Internal server error.")),
                ))
            }
        }))
        .layer(Extension(config))
        .layer(Extension(db_pool))
        .layer(SetResponseHeaderLayer::overriding(
            header::SERVER,
            HeaderValue::from_static("instrumentality"),
        ))
        .timeout(Duration::from_secs(5));

    Router::new()
        .route("/", get(frontpage))
        .route("/types", get(types))
        .route("/login", get(login))
        .route("/view", get(view))
        .route("/queue", get(queue))
        .route("/invite", get(invite))
        .route("/register", post(register))
        .route("/create", post(create))
        .route("/delete", post(delete))
        .route("/update", post(update))
        .route("/add", post(add))
        .layer(service_builder)
        .fallback(default.into_service())
}

fn build_address(address: &str, port: &str) -> SocketAddr {
    format!("{}:{}", address, port).parse().unwrap()
}

async fn build_tls(cert: &str, key: &str) -> RustlsConfig {
    RustlsConfig::from_pem_file(cert, key).await.unwrap()
}
