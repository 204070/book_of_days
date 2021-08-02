use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use self::message::Message;

pub mod message;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: String,
    pub sender: Option<mpsc::UnboundedSender<Message>>,
}
pub type Users = Arc<Mutex<HashMap<String, Client>>>;
