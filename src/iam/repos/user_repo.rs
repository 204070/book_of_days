use std::cell::RefCell;

use crate::iam::types::user::User;

pub trait UserRepository {
    fn save(&self, user: &User);
    fn exist(&self, user: &User) -> bool;
    fn get_by_username(&self, user_name: &str) -> Option<User>;
}

pub struct TestUserRepo {
    users: RefCell<Vec<User>>,
}

impl TestUserRepo {
    pub fn new() -> TestUserRepo {
        TestUserRepo {
            users: RefCell::new(vec![User::new("204070", "Pa55w0rd").unwrap()]),
        }
    }
}

impl UserRepository for TestUserRepo {
    fn save(&self, user: &User) {
        self.users.borrow_mut().push(user.clone());
    }
    fn exist(&self, user: &User) -> bool {
        self.users.borrow().contains(&user)
    }

    fn get_by_username(&self, username: &str) -> Option<User> {
        let user = self
            .users
            .borrow_mut()
            .clone()
            .into_iter()
            .find(|user| user.username == username);
        user
    }
}
