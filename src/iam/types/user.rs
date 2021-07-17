use bcrypt::{hash, verify};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub user_id: Uuid,
    pub password: HashedPassword,
    last_login_time: String,
    last_login_ip: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<User, String> {
        let username = String::from(username);
        let password = String::from(password);
        let password = match HashedPassword::new(password) {
            Ok(hashed_password) => hashed_password,
            Err(err) => return Err(err),
        };

        Ok(User {
            username,
            password,
            user_id: Uuid::new_v4(),
            last_login_time: Utc::now().to_rfc3339(),
            last_login_ip: String::from("0.0.0.0"),
        })
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

#[derive(Clone, Debug)]
pub struct HashedPassword {
    value: String,
}
impl HashedPassword {
    fn new(password: String) -> Result<HashedPassword, String> {
        if !HashedPassword::is_strong_enough(&password) {
            return Err(String::from("Password must be longer than 7 characters"));
        };

        Ok(HashedPassword {
            value: match hash(password, 10) {
                Ok(hashed) => hashed,
                Err(err) => format!("{}", err),
            },
        })
    }

    fn is_strong_enough(password: &str) -> bool {
        password.len() > 7
    }

    pub fn compare(&self, plain_password: &str) -> bool {
        let valid = match verify(plain_password, &self.value) {
            Ok(v) => v,
            Err(_) => false, // TODO: Log Comparison Error
        };
        valid
    }
}
