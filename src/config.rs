//! Functions for interpretation of configurations.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct IConfig {
    pub mongodb: MDBIConfig,
    pub content_types: HashMap<String, Vec<String>>,
    pub presence_types: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct MDBIConfig {
    pub user: String,
    pub password: String,
    pub hosts: String, // This should be an array and be supported as an array in mdb.rs.
    pub port: String,
    pub database: String,
}

pub fn open() -> Result<IConfig, Box<dyn std::error::Error>> {
    let config_str = &std::fs::read_to_string("Instrumentality.toml")
        .expect("Couldn't read Instrumentality.toml");

    let config: IConfig = toml::from_str(config_str)?;
    Ok(config)
}
