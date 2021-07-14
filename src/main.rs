use warp::Filter;

use crate::infra::http::v1;

mod iam;
mod infra;

extern crate pretty_env_logger;
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let v1 = v1::v1_router().with(warp::log("warp::server"));

    warp::serve(v1).run(([0, 0, 0, 0], 7878)).await;
}
