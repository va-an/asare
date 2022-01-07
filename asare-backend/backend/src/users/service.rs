use crate::{users::repository::UserReposotory, UserRepoInMemory};

use super::repository::User;

#[derive(Debug)]
pub struct UserService {
    user_repo: UserRepoInMemory,
}

impl UserService {
    pub fn new(user_repo: UserRepoInMemory) -> UserService {
        UserService { user_repo }
    }

    pub fn create_user(&self) -> User {
        self.user_repo.create()
    }

    pub fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }
}
