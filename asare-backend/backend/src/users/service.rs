use crate::{users::repository::UserReposotory, UserRepoInMemory};

use super::{apikey::ApiKey, repository::User};

#[derive(Debug)]
pub struct UserService {
    user_repo: UserRepoInMemory,
}

impl UserService {
    pub fn new(user_repo: UserRepoInMemory) -> UserService {
        UserService { user_repo }
    }

    pub fn create_user(&self, login: &str, password: &str) -> User {
        let api_key = ApiKey::new();
        let new_user = self.user_repo.create(login, password, &api_key);

        let all_users = self.user_repo.find_all();
        log::debug!("{:#?}", all_users);

        new_user
    }

    pub fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }
}
