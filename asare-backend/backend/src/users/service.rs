use crate::users::generators::{ApiKeyGenerator, UserPasswordGenerator};

use super::{
    repository::{User, UserRepoInMemory, UserReposotory},
    routes::CreateUserRequest,
};

#[derive(Debug)]
pub struct UserService {
    user_repo: UserRepoInMemory,
}

impl UserService {
    pub fn new() -> UserService {
        let user_repo = UserRepoInMemory::new();

        UserService { user_repo }
    }

    pub fn create_user(&self, create_user_request: &CreateUserRequest) -> User {
        let api_key = ApiKeyGenerator::generate();

        let password = match &create_user_request.password {
            Some(password) => password.to_owned(),
            None => UserPasswordGenerator::generate(),
        };

        let new_user = self
            .user_repo
            .create(&create_user_request.login, &password, &api_key);

        let all_users = self.user_repo.find_all();
        log::debug!("{:#?}", all_users);

        new_user
    }

    pub fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }

    pub fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.user_repo.find_by_api_key(api_key)
    }
}
