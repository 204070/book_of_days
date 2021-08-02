use std::fmt;

use crate::iam::repos::user_repo::UserRepository;

pub struct FetchCurrentUserDTO {
    pub user_id: String,
}
pub struct FetchCurrentUserResponse {
    pub user_id: String,
    pub username: String,
}

pub enum FetchCurrentUserError {
    UserDoesNotExist,
    UnexpectedError,
}
impl fmt::Display for FetchCurrentUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchCurrentUserError::UserDoesNotExist => {
                write!(f, "Username or Password does not match")
            }
            FetchCurrentUserError::UnexpectedError => {
                write!(f, "An unexpected error has occured")
            }
        }
    }
}

pub fn execute(
    req: &FetchCurrentUserDTO,
    user_repo: &impl UserRepository,
) -> Result<FetchCurrentUserResponse, FetchCurrentUserError> {
    let user = match user_repo.get_by_user_id(&req.user_id) {
        Some(user) => user,
        None => return Err(FetchCurrentUserError::UserDoesNotExist),
    };

    Ok(FetchCurrentUserResponse {
        user_id: user.id.to_string(),
        username: user.username,
    })
}
