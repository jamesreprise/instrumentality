use instrumentality::rocket::local::asynchronous::Client;

pub async fn setup_client() -> Client {
    Client::untracked(instrumentality::server::build_rocket("InstrumentalityTest.toml").await)
        .await
        .unwrap()
}
