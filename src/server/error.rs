#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("failed to read/write io")]
    IO(std::io::Error),

    #[error("failed to parse object")]
    Serialize(serde_json::Error),
}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::IO(error)
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(error: serde_json::Error) -> Self {
        ServerError::Serialize(error)
    }
}
