use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::telephony::types::message::Message;

pub trait MessageRepository {
    fn save(&self, message: &Message);
}

static MESSAGES: Lazy<Mutex<Vec<Message>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct TestMessageRepo {}

impl TestMessageRepo {
	pub fn new() -> TestMessageRepo {
		TestMessageRepo{}
	}
}

impl MessageRepository for TestMessageRepo {
	fn save(&self, message: &Message) {
		MESSAGES.lock().unwrap().push(message.clone());
	}
}