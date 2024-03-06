use futures::Future;
use warp::{http::StatusCode, reply::Reply};

use super::types::Result;

pub fn health_handler() -> impl Future<Output = Result<impl Reply>> {
    futures::future::ready(Ok(StatusCode::OK))
}
