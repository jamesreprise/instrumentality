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

#[tokio::main]
async fn main() {
    server::build_tracing();

    let config = config::open("Instrumentality.toml").unwrap();
    tracing::info!("Config file loaded.");

    let (app, tls_config, addr) = server::build_server(&config).await;

    let server = axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service());

    tracing::info!("READY: https://{:?}.", addr);
    server.await.unwrap();
}
