use warp::filters::BoxedFilter;
use warp::{body, path, Filter, Reply};

use super::handlers::{login_handler, me_handler, signup_handler};

pub fn iam_router() -> BoxedFilter<(impl Reply,)> {
    iam_prefix().and(signup().or(login().or(me()))).boxed()
}

fn iam_prefix() -> BoxedFilter<()> {
    warp::path("users").boxed()
}

// POST /users/
fn signup() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(body::json())
        .and_then(signup_handler)
        .boxed()
}

// POST /users/login
fn login() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(path("login"))
        .and_then(login_handler)
        .boxed()
}

// GET /users/me
fn me() -> BoxedFilter<(impl Reply,)> {
    warp::get().and(path("me")).and_then(me_handler).boxed()
}
