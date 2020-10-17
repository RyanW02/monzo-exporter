use crate::app::Server;
use std::sync::Arc;
use warp::reply::Response;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{reply, Reply};

pub async fn stats(server: Arc<Server>) -> Result<Response, Infallible> {
    let metrics = &*server.collector.metrics.read().await;

    let mut response = String::new();
    metrics.iter().for_each(|(key, value)| {
        response.push_str(&*value.format(key));
        response.push_str("\n");
    });

    Ok(reply::with_status(response, StatusCode::OK).into_response())
}