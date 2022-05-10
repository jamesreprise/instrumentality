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
    server::build_server("Instrumentality.toml").await;
}
