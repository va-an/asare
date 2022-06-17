use std::collections::HashSet;

use domain::users::User;

pub type UserRepo = Box<dyn UserRepository + Sync + Send>;

pub trait UserRepository {
    fn create(&self, username: &str, password: &str, api_key: &str) -> Result<User, String>;
    fn delete(&self, id: &i32);

    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
    fn find_all_usernames(&self) -> HashSet<String>;
}
