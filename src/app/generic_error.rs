use crate::error::Error;
use warp::http::StatusCode;
use warp::reply::Response;
use warp::{reply, Reply};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GenericError {
    error: Box<str>,
}

impl GenericError {
    pub(crate) fn new(error: Error) -> GenericError {
        GenericError {
            error: format!("{:?}", error).into_boxed_str()
        }
    }

    pub(crate) fn from_str(error: &str) -> GenericError {
        GenericError {
            error: Box::from(error),
        }
    }

    pub(crate) fn into_response(self, status_code: StatusCode) -> Response {
        let mut res = reply::json(&self).into_response();
        *res.status_mut() = status_code;
        res
    }
}