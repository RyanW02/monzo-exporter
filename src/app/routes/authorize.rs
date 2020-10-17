use crate::app::Server;
use std::sync::Arc;
use warp::{reply, Reply};
use warp::http::{StatusCode, header};
use warp::reply::Response;
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Deserialize, Debug)]
pub struct AuthorizeQuery {
    key: Box<str>,
}

pub async fn authorize(server: Arc<Server>, query: AuthorizeQuery) -> Result<Response, Infallible> {
    if server.config.auth_key != query.key {
        return Ok(reply::with_status("Invalid auth key", StatusCode::UNAUTHORIZED).into_response());
    }

    let grant_uri = format!(
        "https://auth.monzo.com/?client_id={}&redirect_uri={}/callback&response_type=code&state={}",
        server.config.client_id, server.config.redirect_uri, server.config.auth_key
    );

    Ok(reply::with_header(
        StatusCode::TEMPORARY_REDIRECT,
        header::LOCATION,
        grant_uri,
    ).into_response())
}
