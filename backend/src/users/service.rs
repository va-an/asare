use std::sync::Arc;

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
}

pub struct UsersImpl {
    user_repo: UserRepo,
}

impl UsersImpl {
    pub fn new(user_repo: UserRepo) -> UserService {
        UsersImpl { user_repo }.pipe(Arc::new)
    }
}

impl Users for UsersImpl {
    /*
    TODO: check uniq username here instead of repo

    1) store usernames in service in Set Usernames
    2) get all usernames when start
        2.1) new fn find_all_usernames() in trait
        2.2) impl fn in trait impls
    3) check user exists in Usernames when create()
    4) add to Usernames when create()
    5) delete from Usernames when delete()

    */
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
