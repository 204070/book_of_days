use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use warp::sse::{self, Event};

use crate::common::handle_reply::WebResult;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Message variants.
#[derive(Debug)]
pub enum Message {
    UserId(usize),
    Reply(String),
}

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

pub async fn chat_send_handler(my_id: usize, msg: String, users: Users) -> WebResult {
	user_message(my_id, msg, &users);
	Ok(Box::new(warp::reply()))
}

pub async fn chat_recv_handler(users: Users) -> WebResult {
	// reply using server-sent events
	let stream = user_connected(users);
	Ok(Box::new(sse::reply(sse::keep_alive().stream(stream))))
}

fn user_connected(
    users: Users,
) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tx.send(Message::UserId(my_id))
        // rx is right above, so this cannot fail
        .unwrap();

    // Save the sender in our list of connected users.
    users.lock().unwrap().insert(my_id, tx);

    // Convert messages into Server-Sent Events and return resulting stream.
    rx.map(|msg| match msg {
        Message::UserId(my_id) => Ok(Event::default().event("user").data(my_id.to_string())),
        Message::Reply(reply) => Ok(Event::default().data(reply)),
    })
}

pub fn user_message(my_id: usize, msg: String, users: &Users) {
    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    users.lock().unwrap().retain(|uid, tx| {
        if my_id == *uid {
            // don't send to same user, but do retain
            true
        } else {
            // If not `is_ok`, the SSE stream is gone, and so don't retain
            tx.send(Message::Reply(new_msg.clone())).is_ok()
        }
    });
}
