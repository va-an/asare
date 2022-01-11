use std::{collections::HashMap, sync::Mutex};

use super::users_service::User;

pub trait UserReposotory {
    fn create(&self, login: &str, password: &str, api_key: &str) -> Result<User, String>;
    fn delete(&self, id: &i32);

    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
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
    fn create(&self, login: &str, password: &str, api_key: &str) -> Result<User, String> {
        let mut all = self.users.lock().unwrap();
        let login_exists = all.values().find(|user| &user.login == login);

        match login_exists {
            Some(_) => Result::Err(format!("User with login '{}' already exists", login)),
            None => {
                let id = self.next_id();
                let user = User {
                    id,
                    login: login.to_owned(),
                    password: password.to_owned(),
                    api_key: api_key.to_owned(),
                };

                all.insert(user.id, user.clone());

                Ok(user)
            }
        }
    }

    fn delete(&self, _id: &i32) {
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

    fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.users
            .lock()
            .unwrap()
            .values()
            .find(|user| user.api_key == api_key)
            .map(|user| user.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::{UserRepoInMemory, UserReposotory};
    use crate::users::generators::ApiKeyGenerator;

    #[test]
    fn create_user() {
        let user_repo = UserRepoInMemory::new();
        let user_1 = user_repo.create("one", "pass1", &ApiKeyGenerator::generate());
        let user_2 = user_repo.create("two", "pass2", &ApiKeyGenerator::generate());

        assert_ne!(user_1, user_2);
    }
}
