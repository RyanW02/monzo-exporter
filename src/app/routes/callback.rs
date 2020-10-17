use crate::app::Server;
use std::sync::Arc;
use warp::reply::Response;
use std::convert::Infallible;
use crate::app::GenericError;
use warp::http::StatusCode;
use crate::monzo;
use warp::{reply, Reply};
use serde::Deserialize;
use crate::monzo::Tokens;

#[derive(Deserialize, Debug)]
pub struct CallbackQuery {
    code: Box<str>,
    state: Box<str>,
}

pub async fn callback(server: Arc<Server>, query: CallbackQuery) -> Result<Response, Infallible> {
    if query.state != server.config.auth_key {
        let res = GenericError::from_str("Invalid auth key");
        return Ok(res.into_response(StatusCode::UNAUTHORIZED));
    }

    // Exchange code for tokens
    let tokens = monzo::exchange_token(&*server.config, query.code).await;
    match tokens {
        Ok(token_response) => {
            let tokens = Tokens::from_token_response(token_response);

            if let Err(e) = tokens.save_to_disk().await {
                let res = GenericError::new(e);
                return Ok(res.into_response(StatusCode::INTERNAL_SERVER_ERROR));
            }

            // TODO: Error handling
            server.monzo_client.update_tokens(tokens).await.unwrap();

            Ok(reply::with_status("Success!", StatusCode::OK).into_response())
        }
        Err(e) => {
            let res = GenericError::new(e);
            Ok(res.into_response(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}