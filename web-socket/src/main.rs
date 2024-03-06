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

    println!("finished...");
}
