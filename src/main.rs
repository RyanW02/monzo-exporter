mod config;
mod error;

mod monzo;
mod app;
use app::Server;

use config::Config;
use std::sync::Arc;
use crate::monzo::{Tokens, Collector};

#[tokio::main]
async fn main() {
    let cfg = Arc::new(Config::parse("config.toml").await.unwrap());

    // TODO: Read tokens.json
    let tokens = match Tokens::read_from_file().await {
        Ok(tokens) => {
            println!("Successfully read tokens.json");
            Some(tokens)
        }
        Err(e) => {
            eprintln!("Error reading tokens from disk ({:?}).\nThis just means you haven't logged in yet: Visit {} to do so", e, cfg.get_authorize_url());
            None
        }
    };

    let monzo_client = Arc::new(monzo::Client::new(cfg.clone(), tokens));
    monzo_client.clone().start_token_loop();

    let collector = Arc::new(Collector::new(monzo_client.clone()));
    collector.clone().start();

    let server = Server::new(cfg.clone(), monzo_client, collector);
    Arc::new(server).start().await.unwrap();
}
