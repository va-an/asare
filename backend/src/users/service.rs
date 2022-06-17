use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use domain::{
    users::{CreateUserRequest, User},
    utils::ChainingExt,
};

use crate::users::generators::{ApiKeyGenerator, UserPasswordGenerator};

use super::repository::UserRepo;

pub type UserService = Arc<dyn Users + Sync + Send>;

pub trait Users {
    fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String>;
    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
    fn delete(&self, username: String);
}

pub struct UsersImpl {
    user_repo: UserRepo,
    usernames: Mutex<HashSet<String>>,
}

impl UsersImpl {
    pub fn new(user_repo: UserRepo) -> UserService {
        let usernames = user_repo.find_all_usernames().pipe(Mutex::new);

        UsersImpl {
            user_repo,
            usernames,
        }
        .pipe(Arc::new)
    }
}

impl Users for UsersImpl {
    fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        if self
            .usernames
            .lock()
            .unwrap()
            .contains(&create_user_request.username)
        {
            let error_msg = format!(
                "User with login '{}' already exists",
                create_user_request.username
            );

            log::error!("{}", error_msg);

            Result::Err(error_msg)
        } else {
            self.usernames
                .lock()
                .unwrap()
                .insert(create_user_request.username.clone());

            let api_key = ApiKeyGenerator::generate();

            let password = match &create_user_request.password {
                Some(password) => password.to_owned(),
                None => UserPasswordGenerator::generate(),
            };

            self.user_repo
                .create(&create_user_request.username, &password, &api_key)
                .tap(|_| log::debug!("created users: {:#?}", self.find_all())) // TODO: delete after debug
        }
    }

    fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all()
    }

    fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.user_repo.find_by_api_key(api_key)
    }

    fn delete(&self, _username: String) {
        // TODO: delete from `usernames` Set when user delete
        // self.user_repo.delete(&username)
        todo!()
    }
}
