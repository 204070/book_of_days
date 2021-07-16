use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, reply, Rejection, Reply};

use super::{repos::user_repo::TestUserRepo, use_cases::create_user};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct SignupResponse {
    pub user_id: String,
}

pub async fn signup_handler(body: SignupRequest) -> WebResult<impl Reply> {
    let user_repo = TestUserRepo::new();
    let user_dto = create_user::CreateUserDTO {
        username: body.username,
        password: body.password,
    };

    let response = match create_user::execute(&user_dto, &user_repo) {
        Ok(user) => Ok(reply::with_status(
            reply::json(&SignupResponse {
                user_id: user.user_id.to_string(),
            }),
            StatusCode::CREATED,
        )),

        Err(e) => match e {
            create_user::CreateUserError::WeakPasswordError(msg) => Ok(reply::with_status(
                reply::json(
                    &vec![("message", msg)]
                        .into_iter()
                        .collect::<HashMap<&str, String>>(),
                ),
                StatusCode::BAD_REQUEST,
            )),
            create_user::CreateUserError::UserAlreadyExistError => Ok(reply::with_status(
                reply::json(
                    &vec![("message", "Username already exist")]
                        .into_iter()
                        .collect::<HashMap<&str, &str>>(),
                ),
                StatusCode::BAD_REQUEST,
            )),
        },
    };

    response
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

#[derive(Serialize)]
struct UserResponse {
    pub user_id: String,
}
pub async fn me_handler() -> WebResult<impl Reply> {
    Ok(reply::json(&UserResponse {
        user_id: String::from("uuid-12345"),
    }))
}
