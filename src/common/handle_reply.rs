use serde::Serialize;
use warp::{hyper::StatusCode, Rejection, Reply};

type WebReply = Box<dyn Reply>;
pub type WebResult = Result<WebReply, Rejection>;

#[derive(Serialize)]
struct Response<T: Serialize> {
    message: String,
    data: T,
}

pub fn reply(message: String, data: impl Serialize, code: StatusCode) -> WebReply {
    Box::new(warp::reply::with_status(
        warp::reply::json(&Response { message, data }),
        code,
    ))
}
