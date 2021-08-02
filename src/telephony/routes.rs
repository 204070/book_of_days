use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use warp::{body, filters::BoxedFilter, Filter, Reply};

use crate::infra::http::middleware::auth_middleware::with_auth;

use super::handlers::{
    fetch_messages_handler, send_message_handler, stream_recv_handler, stream_register_handler,
};

#[derive(Debug)]
struct NotUtf8;
impl warp::reject::Reject for NotUtf8 {}

fn chat_prefix() -> BoxedFilter<()> {
    warp::path("messages").boxed()
}

pub fn chat_router() -> BoxedFilter<(impl Reply,)> {
    // TODO: Save my user_id to env and initiate with my user_id
    // All users currently online
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // POST /messages -> send message
    let message_send = warp::post()
        .and(with_auth())
        .and(body::json())
        .and(users.clone())
        .and_then(send_message_handler)
        .boxed();

    // GET /messages -> messages stream
    let fetch_all_messages = warp::path::end()
        .and(warp::get())
        .and(with_auth())
        .and_then(fetch_messages_handler)
        .boxed();

    // GET /messages/stream/:token
    let stream_recv = warp::path("stream")
        .and(warp::get())
        .and(warp::path::param())
        .and(users)
        .and_then(stream_recv_handler)
        .boxed();

    // GET /messages/register
    let stream_register = warp::path("register")
        .and(warp::post())
        .and(with_auth())
        .and(users)
        .and_then(stream_register_handler)
        .boxed();

    chat_prefix()
        .and(message_send.or(fetch_all_messages).or(stream_recv))
        .boxed()
}
