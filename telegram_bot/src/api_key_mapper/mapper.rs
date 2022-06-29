pub type ApiKeyMapperType = Box<dyn ApiKeyMapper + Send + Sync>;

pub trait ApiKeyMapper {
    fn find_api_key(&self, user_id: u64) -> Option<String>;
    fn create(&self, user_id: u64, api_key: &str);
}
