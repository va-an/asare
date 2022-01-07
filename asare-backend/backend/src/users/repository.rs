use std::{cell::RefCell, collections::HashMap, sync::Mutex};

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct User {
    id: i32,
}

pub trait UserReposotory {
    fn create(&self) -> User;
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
    fn create(&self) -> User {
        let id = self.next_id();
        let user = User { id };

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

    #[test]
    fn create_user() {
        let user_repo = UserRepoInMemory::new();
        let user_1 = user_repo.create();
        let user_2 = user_repo.create();

        assert_ne!(user_1, user_2);
    }
}
