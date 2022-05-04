use crate::config;
use crate::database;
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

pub async fn build_server(config: &str) {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("INFO"))
        .init();

    let config = config::open(config).unwrap();
    tracing::info!("Config file loaded.");
    let db_pool = database::open(&config).await.unwrap();
    tracing::info!("Connected to MongoDB.");

    let service_builder = ServiceBuilder::new()
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
        .layer(Extension(config.clone()))
        .layer(Extension(db_pool))
        .layer(SetResponseHeaderLayer::overriding(
            header::SERVER,
            HeaderValue::from_static("instrumentality"),
        ))
        .timeout(Duration::from_secs(5));

    let app = Router::new()
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
        .fallback(default.into_service())
        .layer(service_builder);

    let tls_config = RustlsConfig::from_pem_file(&config.tls.cert, &config.tls.key)
        .await
        .unwrap();
    tracing::info!("TLS key & cert loaded.");

    let addr: SocketAddr = format!("{}:{}", config.network.address, config.network.port)
        .parse()
        .unwrap();

    tracing::info!("Launching Instrumentality...");
    let server = axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service());

    tracing::info!("READY: https://{:?}.", addr);
    server.await.unwrap();
}
