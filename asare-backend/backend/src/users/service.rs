use crate::users::repository::{UserRepoInMemory, UserReposotory};

use super::repository::User;

trait UserService {
    fn create_user(&self) -> User;
}

struct UserServiceImpl {
    user_repo: Box<dyn UserReposotory>,
}

impl UserServiceImpl {
    fn new(user_repo: Box<dyn UserReposotory>) -> UserServiceImpl {
        UserServiceImpl { user_repo }
    }
}

impl UserService for UserServiceImpl {
    fn create_user(&self) -> User {
        self.user_repo.create()
    }
}
