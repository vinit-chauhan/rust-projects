use tokio::sync::mpsc;
use warp::ws::Message;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use warp::reject::Rejection;

pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
    pub user_id: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterResponse {
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestTopic {
    pub topic: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub topic: String,
    pub user_id: Option<usize>,
    pub message: String,
}

pub type Result<T> = std::result::Result<T, Rejection>;
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
