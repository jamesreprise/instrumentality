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

pub fn open(config_path: &str) -> Result<IConfig, Box<dyn std::error::Error>> {
    let config_str = &std::fs::read_to_string(config_path)
        .unwrap_or_else(|_| panic!("Couldn't read configuration file {}.", config_path));

    let config: IConfig = toml::from_str(config_str)?;
    Ok(config)
}
