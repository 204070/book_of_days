use dotenv::dotenv;
use infra::http::{middleware::rejection_handler::handle_rejection, v1};
use std::env;
use warp::Filter;

mod common;
mod iam;
mod infra;
mod telephony;

extern crate dotenv;
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Authorization", "Content-Type"])
        .allow_methods(vec!["POST", "GET"]);

    let v1 = v1::v1_router()
        .with(cors)
        .with(warp::log("warp::server"))
        .recover(handle_rejection);

    let default_port = 7878;
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port,
    };

    warp::serve(v1).run(([0, 0, 0, 0], port)).await;
}
