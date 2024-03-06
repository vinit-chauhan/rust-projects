use tokio::sync::mpsc;
use warp::ws::Message;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use warp::reject::Rejection;

pub struct Client {
    pub user_id: usize,
    pub topic: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
    user_id: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterResponse {
    url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestTopic {
    topic: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

pub type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
