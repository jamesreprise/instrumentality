//! Functions for the configuration file.

use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::response::Response;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize, Debug)]
pub struct IConfig {
    pub mongodb: MDBIConfig,
    pub content_types: HashMap<String, Vec<String>>,
    pub presence_types: HashMap<String, Vec<String>>,
    pub settings: Settings,
    pub network: NetworkConfig,
    pub tls: TLSConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Settings {
    pub log_level: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct TLSConfig {
    pub cert: String,
    pub key: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NetworkConfig {
    pub address: String,
    pub port: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MDBIConfig {
    pub user: String,
    pub password: String,
    pub hosts: String, // This should be an array.
    pub port: String,
    pub database: String,
}

pub fn open(config_path: &str) -> Result<IConfig, Box<dyn std::error::Error>> {
    let config_str = &std::fs::read_to_string(config_path)?;
    let config: IConfig = toml::from_str(config_str)?;
    Ok(config)
}

#[async_trait]
impl<B: Send> FromRequest<B> for IConfig {
    type Rejection = Response;

    async fn from_request(
        request: &mut RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let config = request.extensions().get::<IConfig>().unwrap();

        Ok(config.clone())
    }
}
