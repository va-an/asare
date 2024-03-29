use async_trait::async_trait;
use domain::users::User;

use super::repository::UserRepository;
use std::{collections::HashMap, sync::Mutex};

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

#[async_trait]
impl UserRepository for UserRepoInMemory {
    async fn create(&self, username: &str, password: &str, api_key: &str) -> Result<User, String> {
        let mut all = self.users.lock().unwrap();

        let user = User {
            id: self.next_id(),
            username: username.to_owned(),
            password: password.to_owned(),
            api_key: api_key.to_owned(),
        };

        all.insert(user.id, user.clone());

        Ok(user)
    }

    async fn delete(&self, _username: &str) -> Result<(), String> {
        todo!()
    }

    async fn find_all(&self) -> Vec<User> {
        self.users.lock().unwrap().values().cloned().collect()
    }

    async fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.users
            .lock()
            .unwrap()
            .values()
            .find(|user| user.api_key == api_key)
            .map(|user| user.to_owned())
    }

    async fn find_all_usernames(&self) -> std::collections::HashSet<String> {
        self.users
            .lock()
            .unwrap()
            .values()
            .map(|user| user.username.to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::users::{generators::ApiKeyGenerator, repository_builder::UserRepositoryBuilder};

    #[tokio::test]
    async fn create_user() {
        let user_repo = UserRepositoryBuilder::in_memory();

        let user_1 = user_repo
            .create("one", "pass1", &ApiKeyGenerator::generate())
            .await;

        let user_2 = user_repo
            .create("two", "pass2", &ApiKeyGenerator::generate())
            .await;

        assert_ne!(user_1, user_2);
    }
}
