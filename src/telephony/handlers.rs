use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::UnboundedReceiverStream;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use warp::{
    hyper::StatusCode,
    sse::{self, Event},
};

use crate::common::handle_reply::{reply, WebResult};

use super::{
    repos::message_repo::TestMessageRepo,
    types::{message::Message, Client, Users},
};

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub message_type: String,
    pub sent_at: String,
    pub value: String,
    pub to: Option<String>, // None unless message from me
}
pub async fn send_message_handler(
    user_id: String,
    body: SendMessageRequest,
    users: Users,
) -> WebResult {
    // Get user from Users
    let message_repo = TestMessageRepo::new();
	return Ok(reply(
		String::from("User ID not found"),
		{},
		StatusCode::BAD_REQUEST,
	))
    Ok(Box::new(warp::reply()))
}

pub async fn stream_recv_handler(user_id: String, users: Users) -> WebResult {
    // reply using server-sent events
    let stream = user_connected(user_id, users);
    Ok(Box::new(sse::reply(sse::keep_alive().stream(stream))))
}

#[derive(Serialize)]
struct FetchMessagesResponse {
    pub user_id: String,
    pub messages: Vec<Message>,
}
pub async fn fetch_messages_handler(user_id: String) -> WebResult {
    Ok(reply(
        String::from("User Fetched"),
        &FetchMessagesResponse {
            user_id,
            messages: vec![],
        },
        StatusCode::OK,
    ))
}

pub async fn stream_register_handler(user_id: String, users: &Users) -> WebResult {
    users.lock().unwrap().insert(
        user_id.clone(),
        Client {
            user_id,
            sender: None,
        },
    );

    Ok(reply(
        String::from("User registered for chat"),
        {},
        StatusCode::OK,
    ))
}

fn user_connected(
    user_id: String,
    users: Users,
) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    // Save the sender in our list of connected users.
    if let Some(client) = users.lock().unwrap().get_mut(&user_id) {
        client.clone().sender = Some(tx);
        users.lock().unwrap().insert(user_id, client.to_owned());
    }

    rx.map(|msg| {
        Ok(Event::default()
            .json_data(msg)
            .expect("Message could not be serialized"))
    })
}
