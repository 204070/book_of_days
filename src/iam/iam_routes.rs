use warp::filters::BoxedFilter;
use warp::{path, Filter, Reply};

use super::iam_handlers::{login_handler, signup_handler};

pub fn iam_router() -> BoxedFilter<(impl Reply,)> {
    iam_prefix().and(signup().or(login())).boxed()
}

fn iam_prefix() -> BoxedFilter<()> {
    warp::path("iam").boxed()
}

// POST /iam/signup
fn signup() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("signup"))
        .and_then(signup_handler)
        .boxed()
}

// POST /iam/login
fn login() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("login"))
        .and_then(login_handler)
        .boxed()
}

// GET /users/me
// fn me() -> BoxedFilter<(impl Reply,)> {
//     warp::get().and(path("me")).and_then(me_handler).boxed()
// }

// #[derive(Serialize)]
// struct UserResponse {
//     pub user_id: String,
// }
// async fn me_handler() -> WebResult<impl Reply> {
//     Ok(reply::json(&UserResponse {
//         user_id: String::from("uuid-12345"),
//     }))
// }