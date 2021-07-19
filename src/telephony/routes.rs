use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use warp::{filters::BoxedFilter, hyper::body::Bytes, Filter,  Reply};

use super::handlers::{chat_recv_handler, chat_send_handler};

#[derive(Debug)]
struct NotUtf8;
impl warp::reject::Reject for NotUtf8 {}

fn chat_prefix() -> BoxedFilter<()> {
    warp::path("chat").boxed()
}

pub fn chat_router() -> BoxedFilter<(impl Reply,)> {
    // Keep track of all connected users, key is usize, value
    // is an event stream sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // POST /chat -> send message
    let chat_send = warp::post()
        .and(warp::path::param::<usize>())
        .and(warp::body::content_length_limit(500))
        .and(warp::body::bytes().and_then(|body: Bytes| async move {
            std::str::from_utf8(&body)
                .map(String::from)
                .map_err(|_e| warp::reject::custom(NotUtf8))
        }))
        .and(users.clone())
        .and_then(chat_send_handler)
        .boxed();

    // GET /chat -> messages stream
    let chat_recv = warp::get().and(users).and_then(chat_recv_handler).boxed();

    chat_prefix().and(chat_send.or(chat_recv)).boxed()
}
