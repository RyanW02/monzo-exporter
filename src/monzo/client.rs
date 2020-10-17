use crate::config::Config;
use super::model::ExchangeBody;
use super::Tokens;
use crate::error::Error;
use crate::monzo::model::{TokenResponse, GetBalance};
use tokio::sync::{mpsc, Mutex, RwLock};
use std::sync::Arc;
use tokio::time::delay_for;
use chrono::{Duration, Utc};
use tokio::sync::mpsc::error::SendError;

pub const BASE_URL: &str = "https://api.monzo.com";

pub struct Client {
    config: Arc<Config>,
    update_tokens_tx: mpsc::Sender<Tokens>,
    update_tokens_rx: Mutex<mpsc::Receiver<Tokens>>,
    tokens: RwLock<Option<Tokens>>,
}

impl Client {
    pub fn new(config: Arc<Config>, tokens: Option<Tokens>) -> Client {
        let (update_tokens_tx, update_tokens_rx) = mpsc::channel(1);

        Client {
            config,
            update_tokens_tx,
            update_tokens_rx: Mutex::new(update_tokens_rx),
            tokens: RwLock::new(tokens),
        }
    }

    pub async fn update_tokens(&self, tokens: Tokens) -> Result<(), SendError<Tokens>> {
        self.update_tokens_tx.clone().send(tokens).await
    }

    pub fn start_token_loop(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut update_rx = self.update_tokens_rx.lock().await;

            loop {
                let mut mutex = self.tokens.write().await;

                match *mutex {
                    Some(ref mut tokens) => {
                        if tokens.expires_at.signed_duration_since(Utc::now()) < Duration::zero() {
                            *mutex = None;
                            println!("Your tokens have expired: Visit {} to login again", self.config.get_authorize_url());
                            continue;
                        }

                        let until_refresh = tokens.expires_at - Utc::now() - Duration::minutes(30);

                        drop(mutex);

                        tokio::select! {
                            new = update_rx.recv() => {
                                *self.tokens.write().await = new;
                                println!("Received new tokens");
                            }

                            _ = delay_for(until_refresh.to_std().unwrap_or(Duration::zero().to_std().unwrap())) => {
                                println!("Doing token refresh");

                                if let Some(ref mut tokens) = &mut *self.tokens.write().await {
                                    // Panic if refresh failed
                                    if let Err(e) = tokens.do_refresh(&*self.config).await {
                                        Tokens::delete_file().await.unwrap();
                                        panic!("Error while refreshing tokens: {:?}", e);
                                    }
                                } else {
                                    println!("Your tokens have expired: Visit {} to login again", self.config.get_authorize_url());
                                }
                            }
                        }
                    }

                    None => {
                        let updated = update_rx.recv().await;
                        *mutex = updated;
                        println!("Received new tokens");
                    }
                }
            }
        });
    }

    pub async fn get_balance(&self) -> Result<GetBalance, Error> {
        let access_token = match &*self.tokens.read().await {
            Some(token) => token.access_token.clone(),
            None => return Error::NotLoggedIn.into()
        };

        let endpoint = format!("{}/balance?account_id={}", BASE_URL, self.config.account_id);
        let auth_header = format!("Bearer {}", access_token);

        let res = reqwest::Client::new()
            .get(&endpoint)
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .send()
            .await.map_err(Error::HTTPError)?
            .json::<GetBalance>().await
            .map_err(Error::HTTPError)?;

        Ok(res)
    }
}

pub async fn exchange_token(config: &Config, code: Box<str>) -> Result<TokenResponse, Error> {
    let endpoint = format!("{}/oauth2/token", BASE_URL);
    let body = ExchangeBody::new(
        config.client_id.clone(),
        config.client_secret.clone(),
        format!("{}/callback", config.redirect_uri).into_boxed_str(),
        code,
    );

    let res = reqwest::Client::new()
        .post(&endpoint)
        .form(&body)
        .send()
        .await.map_err(Error::HTTPError)?;

    res.json().await.map_err(Error::HTTPError)
}
