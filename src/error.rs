#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error occurred while performing an IO operation: {0}")]
    IOError(#[from] tokio::io::Error),

    #[error("An error occurred while parsing the config: {0}")]
    TOMLError(#[from] toml::de::Error),

    #[error("An error occurred while parsing environment variables: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("An error occurred while parsing performing a HTTP request: {0}")]
    HTTPError(#[from] reqwest::Error),

    #[error("An error occurred while encoding JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("User hasn't authorized yet")]
    NotLoggedIn,
}

impl<T> Into<Result<T, Error>> for Error {
    fn into(self) -> Result<T, Self> {
        Err(self)
    }
}