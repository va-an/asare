use crate::app::UsersType;
use std::sync::Arc;

pub struct UserApiKeyMatcher {
    users: UsersType,
}

impl UserApiKeyMatcher {
    pub fn new(users: UsersType) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            users: Arc::clone(&users),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.users.find_by_api_key(api_key).map(|user| user.id)
    }
}
