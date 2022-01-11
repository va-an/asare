use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    app::UsersType,
    users::{
        generators::{ApiKeyGenerator, UserPasswordGenerator},
        repository::UserReposotory,
    },
    utils::ChainingExt,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String, // FIXME: store hash instead raw password
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub login: String,
    pub password: Option<String>,
}

pub trait Users {
    fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String>;
    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
}

type UserRepo = Box<dyn UserReposotory + Sync + Send>;

pub struct UsersImpl {
    pub user_repo: UserRepo,
}

impl UsersImpl {
    pub fn new(user_repo: UserRepo) -> UsersType {
        UsersImpl { user_repo }.pipe(Arc::new)
    }
}

impl Users for UsersImpl {
    fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        let api_key = ApiKeyGenerator::generate();

        let password = match &create_user_request.password {
            Some(password) => password.to_owned(),
            None => UserPasswordGenerator::generate(),
        };

        self.user_repo
            .create(&create_user_request.login, &password, &api_key)
            .tap(|| log::debug!("created users: {:#?}", self.find_all()))
    }

    fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }

    fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.user_repo.find_by_api_key(api_key)
    }
}
