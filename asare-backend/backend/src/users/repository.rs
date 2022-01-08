use serde::Serialize;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct User {
    id: i32,
    login: String,
    password: String, // FIXME: store hash instead raw password
    api_key: String,
}

pub trait UserReposotory {
    fn create(&self, login: &str, password: &str, api_key: &str) -> User;
    fn delete(&self, id: &i32);
    fn find_all(&self) -> Vec<User>;
}

#[derive(Debug)]
pub struct UserRepoInMemory {
    users: Mutex<HashMap<i32, User>>,
    id_counter: Mutex<i32>,
}

impl UserRepoInMemory {
    pub fn new() -> UserRepoInMemory {
        UserRepoInMemory {
            users: Mutex::new(HashMap::new()),
            id_counter: Mutex::new(0),
        }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.lock().unwrap() + 1;
        *self.id_counter.lock().unwrap() = next;

        next
    }
}

impl UserReposotory for UserRepoInMemory {
    fn create(&self, login: &str, password: &str, api_key: &str) -> User {
        let id = self.next_id();
        let user = User {
            id,
            login: login.to_owned(),
            password: password.to_owned(),
            api_key: api_key.to_owned(),
        };

        self.users.lock().unwrap().insert(user.id, user.clone());

        user
    }

    fn delete(&self, id: &i32) {
        todo!()
    }

    fn find_all(&self) -> Vec<User> {
        self.users
            .lock()
            .unwrap()
            .values()
            .map(|user| user.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{UserRepoInMemory, UserReposotory};
    use crate::users::apikey::ApiKey;

    #[test]
    fn create_user() {
        let user_repo = UserRepoInMemory::new();
        let user_1 = user_repo.create("one", "pass1", &ApiKey::new());
        let user_2 = user_repo.create("two", "pass2", &ApiKey::new());

        assert_ne!(user_1, user_2);
    }
}
