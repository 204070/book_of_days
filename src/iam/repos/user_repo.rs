use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::iam::types::user::User;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;

pub trait UserRepository {
    fn save(&self, user: &User);
    fn exist(&self, user: &User) -> bool;
    fn get_by_username(&self, user_name: &str) -> Option<User>;
    fn get_by_user_id(&self, user_id: &str) -> Option<User>;
}

static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| {
    let me = User::new("204070", "Pa55w0rd").unwrap();
    let id = env::var("DEFAULT_UUID").expect("Error getting DEFAULT_UUID from env");
    me.id = Uuid::from_str(&id).expect("Uuid gen panic");
    Mutex::new(vec![me])
});

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
    fn get_by_user_id(&self, user_id: &str) -> Option<User> {
        let user_id = Uuid::from_str(user_id).expect("Error parsing user_id as Uuid");
        let user = USERS
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .find(|user| user.id == user_id);
        user
    }
}
