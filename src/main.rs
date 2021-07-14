use warp::Filter;

use crate::infra::http::v1;
use std::env;

mod iam;
mod infra;

extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let v1 = v1::v1_router().with(warp::log("warp::server"));

    let default_port = 7878;
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap_or(default_port),
        Err(_) => default_port,
    };

    warp::serve(v1).run(([0, 0, 0, 0], port)).await;
}
