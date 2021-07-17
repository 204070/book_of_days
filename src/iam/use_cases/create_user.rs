use core::fmt;

use crate::iam::{repos::user_repo::UserRepository, types::user::User};

pub struct CreateUserDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum CreateUserError {
    WeakPassword(String),
    UserAlreadyExist,
}

impl fmt::Display for CreateUserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateUserError::UserAlreadyExist => write!(f, "User already exist"),
            CreateUserError::WeakPassword(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn execute(
    req: &CreateUserDTO,
    user_repo: &impl UserRepository,
) -> Result<User, CreateUserError> {
    if user_repo.get_by_username(&req.username) != None {
        return Err(CreateUserError::UserAlreadyExist);
    }
    let user =
        User::new(&req.username, &req.password).map_err(|e| CreateUserError::WeakPassword(e))?;

    user_repo.save(&user);

    Ok(user)
}

#[cfg(test)]
mod create_user_tests {
    use crate::iam::repos::user_repo::TestUserRepo;

    use super::*;

    #[test]
    fn it_works() {
        let user_repo = TestUserRepo::new();
        let user_dto = CreateUserDTO {
            username: String::from("test"),
            password: String::from("Pa55w0rd"),
        };

        let result = execute(&user_dto, &user_repo);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert!(user_repo.exist(&user));
    }

    #[test]
    fn weak_password_fails() {
        let user_repo = TestUserRepo::new();
        let user_dto = CreateUserDTO {
            username: String::from("test"),
            password: String::from("Pass"),
        };

        let result = execute(&user_dto, &user_repo);
        assert!(result.is_err());

        let result = result.unwrap_err();
        assert!(match result {
            CreateUserError::WeakPassword(_) => true,
            _ => panic!("Error not correct"),
        });
    }

    #[test]
    fn already_existing_username_fails() {
        let user_repo = TestUserRepo::new();
        let user_dto = CreateUserDTO {
            username: String::from("204070"),
            password: String::from("Pass"),
        };

        let result = execute(&user_dto, &user_repo);
        assert!(result.is_err());

        let result = result.unwrap_err();
        assert!(match result {
            CreateUserError::UserAlreadyExist => true,
            _ => panic!("Error not correct"),
        });
    }
}
