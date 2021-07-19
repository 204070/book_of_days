use std::fmt;

use crate::iam::{repos::user_repo::UserRepository, services::auth_service};

pub struct GenerateAuthTokensDTO {
    pub username: String,
    pub password: String,
}
pub struct GenerateAuthTokensDTOResponse {
    pub access_token: auth_service::JWTToken,
    pub refresh_token: String,
}

pub enum GenerateAuthTokensError {
    UserDoesNotExist,
    PasswordDoesNotMatch,
    UnexpectedError,
}
impl fmt::Display for GenerateAuthTokensError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateAuthTokensError::UserDoesNotExist => {
                write!(f, "Username or Password does not match")
            }
            GenerateAuthTokensError::PasswordDoesNotMatch => {
                write!(f, "Username or Password does not match")
            }
            GenerateAuthTokensError::UnexpectedError => {
                write!(f, "An unexpected error has occured")
            }
        }
    }
}

pub fn execute(
    req: &GenerateAuthTokensDTO,
    user_repo: &impl UserRepository,
) -> Result<GenerateAuthTokensDTOResponse, GenerateAuthTokensError> {
    // Fetch User
    let user = match user_repo.get_by_username(&req.username) {
        Some(user) => user,
        None => return Err(GenerateAuthTokensError::UserDoesNotExist),
    };
    // TODO if guest, create throwaway account

    // Compare Password
    let password_valid = user.password.compare(&req.password);
    if password_valid == false {
        return Err(GenerateAuthTokensError::PasswordDoesNotMatch);
    }
    // If password match, generate jwt and refresh tokens
    let access_token = auth_service::sign_jwt(&user.username)
        .map_err(|_| GenerateAuthTokensError::UnexpectedError)?;
    let refresh_token = String::from("refresh_token");
    Ok(GenerateAuthTokensDTOResponse {
        access_token,
        refresh_token,
    })
}

#[cfg(test)]
mod generate_auth_tokens_test {
    use super::*;
    use crate::iam::repos::user_repo::TestUserRepo;

    #[test]
    fn it_works() {
        let user_repo = TestUserRepo::new();
        let user_dto = GenerateAuthTokensDTO {
            username: String::from("204070"),
            password: String::from("Pa55w0rd"),
        };

        let result = execute(&user_dto, &user_repo);
        assert!(result.is_ok());
    }
}
