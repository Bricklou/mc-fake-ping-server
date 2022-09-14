use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use super::error::ConfigError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_server_host")]
    pub host: String,
    #[serde(default = "default_server_port")]
    pub port: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 25565,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = Path::new("./config.toml");

        let data = fs::read_to_string(path).unwrap_or(String::new());
        let config = toml::from_str::<Config>(&data)?;

        config.save()?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Path::new("./config.toml");

        let str = toml::to_string(&self)?;
        let mut file = File::create(path)?;
        file.write_all(str.as_bytes())?;

        Ok(())
    }
}

fn default_server_host() -> String {
    "0.0.0.0".to_string()
}
fn default_server_port() -> u32 {
    25565
}
