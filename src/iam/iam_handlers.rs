use serde::Serialize;
use warp::{reply, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
struct SignupResponse {
    pub user_id: String,
}
pub async fn signup_handler() -> WebResult<impl Reply> {
    Ok(reply::json(&SignupResponse {
        user_id: String::from("uuid-12345"),
    }))
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}
pub async fn login_handler() -> WebResult<impl Reply> {
    Ok(reply::json(&LoginResponse {
        access_token: String::from("uuid-12345"),
        refresh_token: String::from("uuid-12345"),
    }))
}
