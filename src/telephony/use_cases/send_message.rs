use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

use crate::telephony::{
    repos::message_repo::MessageRepository,
    types::{
        message::{Message, MessageType},
        Users,
    },
};

pub struct SendMessageDTO {
    pub user_id: String,
    pub message_type: String,
    pub sent_at: String,
    pub value: String,
    pub to: Option<String>, // None unless message from me
    users: Users,
}

pub enum SendMessageError {
    UserIDNotFound,
    InvalidSentAt,
    UnexpectedError(String),
}

pub fn execute(
    req: &SendMessageDTO,
    message_repo: &impl MessageRepository,
) -> Result<(), SendMessageError> {
    let users = req.users;
    let client = match users.lock().unwrap().get(&req.user_id) {
        Some(client) => client,
        None => return Err(SendMessageError::UserIDNotFound),
    };
    let user_id = Uuid::from_str(&req.user_id).expect("Error parsing user_id as Uuid");
    let message_type = match &req.message_type[..] {
        "text" => MessageType::Text,
    };
    let sent_at = DateTime::<Utc>::from_str(&req.sent_at[..])
        .map_err(|_err| return SendMessageError::InvalidSentAt)?;
    let message = Message::new(user_id, req.value, message_type, sent_at)
        .map_err(|err| return SendMessageError::UnexpectedError(err))?;

    message_repo.save(&message);
    // Send message to me/friend

	
    // New message from this user, send it to everyone else (except same uid)...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    users.lock().unwrap().retain(|uid, client| {
        if req.user_id == *uid {
            // don't send to same user, but do retain
            true
        } else {
            // If not `is_ok`, the SSE stream is gone, and so don't retain
            client.sender.unwrap().send(message).is_ok()
        }
    });
    Ok({})
}
