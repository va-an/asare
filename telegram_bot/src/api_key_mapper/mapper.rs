pub type UserId = i64;

pub type ApiKeyMapperType = Box<dyn ApiKeyMapper + Send + Sync>;

pub trait ApiKeyMapper {
    fn find_api_key(&self, user_id: UserId) -> Option<String>;
    fn create(&mut self, user_id: UserId, api_key: &str);
}
