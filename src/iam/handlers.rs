use crate::common::handle_reply::{reply, WebResult};
use serde::{Deserialize, Serialize};
use warp::hyper::StatusCode;

use super::{
    repos::user_repo::TestUserRepo,
    use_cases::{create_user, fetch_current_user, generate_auth_tokens},
};

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
                user_id: user.id.to_string(),
            },
            StatusCode::CREATED,
        )),
        Err(e) => Ok(reply(e.to_string(), {}, StatusCode::BAD_REQUEST)),
    };

    response
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}
pub async fn login_handler(body: LoginRequest) -> WebResult {
    let user_repo = TestUserRepo::new();
    let login_dto = generate_auth_tokens::GenerateAuthTokensDTO {
        username: body.username,
        password: body.password,
    };

    let response = match generate_auth_tokens::execute(&login_dto, &user_repo) {
        Ok(tokens) => Ok(reply(
            String::from("Auth Token Generated"),
            &LoginResponse {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
            },
            StatusCode::CREATED,
        )),
        Err(e) => Ok(reply(e.to_string(), {}, StatusCode::BAD_REQUEST)),
    };

    response
}

#[derive(Serialize)]
struct UserResponse {
    pub user_id: String,
    pub username: String,
}
pub async fn me_handler(user_id: String) -> WebResult {
    let user_repo = TestUserRepo::new();
    let fetch_current_user_dto = fetch_current_user::FetchCurrentUserDTO { user_id };
    let response = match fetch_current_user::execute(&fetch_current_user_dto, &user_repo) {
        Ok(user) => Ok(reply(
            String::from("Loggedin user data fetched"),
            &UserResponse {
                user_id: user.user_id,
                username: user.username,
            },
            StatusCode::OK,
        )),
        Err(e) => Ok(reply(e.to_string(), {}, StatusCode::BAD_REQUEST)),
    };

    response
}
