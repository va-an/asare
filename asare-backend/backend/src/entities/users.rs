use serde::{Deserialize, Serialize};

use crate::users::{
    generators::{ApiKeyGenerator, UserPasswordGenerator},
    repository::{UserRepoInMemory, UserReposotory},
};

#[derive(Debug)]
pub struct Users {
    user_repo: UserRepoInMemory,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub login: String,
    pub password: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String, // FIXME: store hash instead raw password
    pub api_key: String,
}

impl Users {
    pub fn new() -> Users {
        let user_repo = UserRepoInMemory::new();

        Users { user_repo }
    }

    // FIXME: make login field unique
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

        // FIXME: delete logging before release
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
