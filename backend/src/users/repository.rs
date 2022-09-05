use std::collections::HashSet;

use async_trait::async_trait;
use domain::users::User;

pub type UserRepo = Box<dyn UserRepository + Sync + Send>;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, username: &str, password: &str, api_key: &str) -> Result<User, String>;
    async fn delete(&self, username: &str);

    async fn find_all(&self) -> Vec<User>;
    async fn find_by_api_key(&self, api_key: &str) -> Option<User>;
    async fn find_all_usernames(&self) -> HashSet<String>;
}
