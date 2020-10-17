use tokio::fs;
use serde::Deserialize;
use crate::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub account_id: Box<str>,
    pub client_id: Box<str>,
    pub client_secret: Box<str>,
    pub server_addr: Box<str>,
    pub redirect_uri: Box<str>,
    pub auth_key: Box<str>,
}

impl Config {
    pub async fn parse(file_name: &str) -> Result<Config, Error> {
        let bytes = fs::read(file_name).await.map_err(Error::IOError)?;
        let config = toml::from_slice(&bytes[..]).map_err(Error::TOMLError)?;
        Ok(config)
    }

    pub fn get_authorize_url(&self) -> String {
        format!("{}/authorize?key={}", self.redirect_uri, self.auth_key)
    }
}