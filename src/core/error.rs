#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read/write the config file")]
    IO(std::io::Error),
    #[error("failed to parse the config file")]
    TomlRead(toml::de::Error),
    #[error("failed to serialize the config")]
    TomlWrite(toml::ser::Error),
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IO(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::TomlRead(error)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(error: toml::ser::Error) -> Self {
        ConfigError::TomlWrite(error)
    }
}
