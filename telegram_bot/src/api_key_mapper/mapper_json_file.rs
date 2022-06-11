use std::{collections::HashMap, sync::Mutex};

use super::mapper::{ApiKeyMapper, UserId};

pub struct ApiKeyMapperJsonFile {
    values: Mutex<HashMap<UserId, String>>,
}

impl ApiKeyMapperJsonFile {
    pub fn new() -> Self {
        ApiKeyMapperJsonFile {
            values: Mutex::new(HashMap::new()),
        }
    }
}

impl ApiKeyMapper for ApiKeyMapperJsonFile {
    fn find_api_key(&self, user_id: UserId) -> Option<String> {
        self.values
            .lock()
            .unwrap()
            .get(&user_id)
            .map(|api_key| api_key.to_owned())
    }

    // TODO: return Result?
    fn create(&self, user_id: UserId, api_key: &str) {
        self.values
            .lock()
            .unwrap()
            .insert(user_id, api_key.to_string());
    }
}
