use serde::{Serialize, Deserialize};
use crate::error::Error;
use tokio::fs;
use crate::monzo::model::{TokenResponse, RefreshBody};
use chrono::{DateTime, Utc, Duration};
use crate::monzo::BASE_URL;
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub access_token: Box<str>,
    pub expires_at: DateTime<Utc>,
    pub refresh_token: Box<str>,
}

const TOKENS_PATH: &str = "tokens.json";

impl Tokens {
    pub fn from_token_response(token_response: TokenResponse) -> Tokens {
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in as i64);

        Tokens {
            access_token: token_response.access_token,
            expires_at,
            refresh_token: token_response.refresh_token,
        }
    }

    pub fn update_from_token_response(&mut self, token_response: TokenResponse) {
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in as i64);

        self.access_token = token_response.access_token;
        self.refresh_token = token_response.refresh_token;
        self.expires_at = expires_at;
    }

    pub async fn save_to_disk(&self) -> Result<(), Error> {
        let json = serde_json::to_string_pretty(&self).map_err(Error::JsonError)?;
        fs::write(TOKENS_PATH, json).await.map_err(Error::IOError)
    }

    pub async fn read_from_file() -> Result<Self, Error> {
        let bytes = fs::read(TOKENS_PATH).await.map_err(Error::IOError)?;
        serde_json::from_slice(&bytes[..]).map_err(Error::JsonError)
    }

    pub async fn delete_file() -> Result<(), Error> {
        fs::remove_file(TOKENS_PATH).await.map_err(Error::IOError)
    }

    pub async fn do_refresh(&mut self, config: &Config) -> Result<(), Error> {
        let endpoint = format!("{}/oauth2/token", BASE_URL);
        let body = RefreshBody::new(
            config.client_id.clone(),
            config.client_secret.clone(),
            self.refresh_token.clone()
        );

        let res = reqwest::Client::new()
            .post(&endpoint)
            .form(&body)
            .send()
            .await.map_err(Error::HTTPError)?
            .json::<TokenResponse>().await
            .map_err(Error::HTTPError)?;

        self.update_from_token_response(res);
        self.save_to_disk().await?;

        Ok(())
    }
}