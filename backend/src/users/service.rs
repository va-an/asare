use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use argon2::Config;
use async_trait::async_trait;
use domain::{
    users::{CreateUserRequest, User},
    utils::ChainingExt,
};
use rand::Rng;

use crate::users::generators::{ApiKeyGenerator, UserPasswordGenerator};

use super::repository::UserRepo;

pub type UserService = Arc<dyn Users + Sync + Send>;

#[async_trait]
pub trait Users {
    async fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String>;
    async fn find_all(&self) -> Vec<User>;
    async fn find_by_api_key(&self, api_key: &str) -> Option<User>;
    async fn delete(&self, username: String) -> Result<(), String>;
}

pub struct UsersImpl {
    user_repo: UserRepo,
    usernames: Mutex<HashSet<String>>,
}

impl UsersImpl {
    pub async fn new(user_repo: UserRepo) -> UserService {
        user_repo
            .find_all_usernames()
            .await
            .pipe(Mutex::new)
            .pipe(|usernames| UsersImpl {
                user_repo,
                usernames,
            })
            .pipe(Arc::new)
    }
}

#[async_trait]
impl Users for UsersImpl {
    async fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        let is_user_exists = self
            .usernames
            .lock()
            .unwrap()
            .contains(&create_user_request.username);

        if is_user_exists {
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

            let password_encoded = {
                let password = match &create_user_request.password {
                    Some(password) => password.to_owned(),
                    None => UserPasswordGenerator::generate(),
                };

                let salt = rand::thread_rng().gen::<[u8; 32]>();
                let config = Config::default();

                argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
            };

            self.user_repo
                .create(&create_user_request.username, &password_encoded, &api_key)
                .await
        }
    }

    async fn find_all(&self) -> Vec<User> {
        self.user_repo.find_all().await
    }

    async fn find_by_api_key(&self, api_key: &str) -> Option<User> {
        self.user_repo.find_by_api_key(api_key).await
    }

    async fn delete(&self, username: String) -> Result<(), String> {
        self.usernames.lock().unwrap().remove(&username);
        self.user_repo.delete(&username).await
    }
}
