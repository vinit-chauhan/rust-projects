mod handler;
mod types;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use types::Clients;
use warp::Filter;

#[tokio::main]
async fn main() {
    let client: Clients = Arc::new(Mutex::new(HashMap::new()));
    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_route = register
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::register_handler);
}
