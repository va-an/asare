use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: i32,
}

pub trait UserReposotory {
    fn create(&self) -> User;
    fn delete(&self, id: &i32);
}

pub struct UserRepoInMemory {
    users: RefCell<HashMap<i32, User>>,
    id_counter: RefCell<i32>,
}

impl UserRepoInMemory {
    fn new() -> UserRepoInMemory {
        UserRepoInMemory {
            users: RefCell::new(HashMap::new()),
            id_counter: RefCell::new(0),
        }
    }

    fn next_id(&self) -> i32 {
        let next = *self.id_counter.borrow() + 1;
        *self.id_counter.borrow_mut() = next;

        next
    }
}

impl UserReposotory for UserRepoInMemory {
    fn create(&self) -> User {
        let id = self.next_id();
        let user = User { id };

        self.users.borrow_mut().insert(user.id, user.clone());

        user
    }

    fn delete(&self, id: &i32) {
        todo!()
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
