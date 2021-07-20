use once_cell::sync::Lazy;

use std::sync::Mutex;

use crate::iam::types::user::User;

pub trait UserRepository {
    fn save(&self, user: &User);
    fn exist(&self, user: &User) -> bool;
    fn get_by_username(&self, user_name: &str) -> Option<User>;
}

static USERS: Lazy<Mutex<Vec<User>>> =
    Lazy::new(|| Mutex::new(vec![User::new("204070", "Pa55w0rd").unwrap()]));

pub struct TestUserRepo {}

impl TestUserRepo {
    pub fn new() -> TestUserRepo {
        TestUserRepo {}
    }
}

impl UserRepository for TestUserRepo {
    fn save(&self, user: &User) {
        USERS.lock().unwrap().push(user.clone());
    }
    fn exist(&self, user: &User) -> bool {
        USERS.lock().unwrap().contains(&user)
    }

    fn get_by_username(&self, username: &str) -> Option<User> {
        let user = USERS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .find(|user| user.username == username);
        user
    }
}
