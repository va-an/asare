use super::users_service::User;

pub type UserRepo = Box<dyn UserReposotory + Sync + Send>;

pub trait UserReposotory {
    fn create(&self, login: &str, password: &str, api_key: &str) -> Result<User, String>;
    fn delete(&self, id: &i32);

    fn find_all(&self) -> Vec<User>;
    fn find_by_api_key(&self, api_key: &str) -> Option<User>;
}