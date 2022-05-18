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

use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    server::build_tracing();

    let config = config::open("Instrumentality.toml");
    if let Ok(config) = config {
        tracing::info!("Config file loaded.");

        let (app, tls_config, addr) = server::build_server(&config).await;

        let server = axum_server::bind_rustls(addr, tls_config).serve(app.into_make_service());

        tracing::info!("READY: https://{:?}.", addr);
        server.await.unwrap();
    } else {
        tracing::info!("Couldn't load \"Instrumentality.toml\", creating an example at InstrumentalityExample.toml.");
        let mut file = File::create("InstrumentalityExample.toml").unwrap();
        file.write_all(EXAMPLE_CONFIG_FILE).unwrap();
    }
}

const EXAMPLE_CONFIG_FILE: &[u8] = b"[content_types]
instagram = [\"post\", \"story\", \"live\"]
twitter = [\"tweet\", \"like\", \"retweet\", \"story\"]
last_fm = [\"scrobble\"]
twitch_tv = [\"stream_start\", \"video\", \"stream_end\"]

[presence_types]
twitter = [\"follower_count_increase\"]
last_fm = [\"now_playing\"]
twitch_tv = [\"live\"]

[mongodb]
hosts = \"127.0.0.1\"
port = \"27017\"
user = \"instrumentality\"
password = \"\"
database = \"instrumentality\"

[settings]
log_level = \"INFO\"

[network]
address = \"127.0.0.1\"
port = \"12321\"

[tls]
cert = \"tls/cert.pem\"
key = \"tls/privkey.pem\"";
