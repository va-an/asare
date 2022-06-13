use std::sync::Arc;

use domain::{users::{User, CreateUserRequest}, utils::ChainingExt};

use crate::users::generators::{ApiKeyGenerator, UserPasswordGenerator};

use super::repository::UserRepo;

pub type UsersService = Arc<dyn Users + Sync + Send>;

pub trait Users {
    fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String>;
    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
}

pub struct UsersImpl {
    pub user_repo: UserRepo,
}

impl UsersImpl {
    pub fn new(user_repo: UserRepo) -> UsersService {
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
            .tap(|_| log::debug!("created users: {:#?}", self.find_all()))
    }

    fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }

    fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.user_repo.find_by_api_key(api_key)
    }
}
