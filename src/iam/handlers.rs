use crate::common::handle_reply::{reply, WebResult};
use serde::{Deserialize, Serialize};
use warp::hyper::StatusCode;

use super::{repos::user_repo::TestUserRepo, use_cases::create_user};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct SignupResponse {
    pub user_id: String,
}

pub async fn signup_handler(body: SignupRequest) -> WebResult {
    let user_repo = TestUserRepo::new();
    let user_dto = create_user::CreateUserDTO {
        username: body.username,
        password: body.password,
    };

    let response = match create_user::execute(&user_dto, &user_repo) {
        Ok(user) => Ok(reply(
            String::from("User Created"),
            &SignupResponse {
                user_id: user.user_id.to_string(),
            },
            StatusCode::CREATED,
        )),
        Err(e) => Ok(reply(format!("{}", e), {}, StatusCode::BAD_REQUEST)),
    };

    response
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}
pub async fn login_handler() -> WebResult {
    Ok(reply(
        String::from("New Access Token created"),
        &LoginResponse {
            access_token: String::from("uuid-12345"),
            refresh_token: String::from("uuid-12345"),
        },
        StatusCode::NOT_IMPLEMENTED,
    ))
}

#[derive(Serialize)]
struct UserResponse {
    pub user_id: String,
}
pub async fn me_handler() -> WebResult {
    Ok(reply(
        String::from("User Fetched"),
        &UserResponse {
            user_id: String::from("uuid-12345"),
        },
        StatusCode::NOT_IMPLEMENTED,
    ))
}
