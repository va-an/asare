use crate::entities::users::{Users, UsersImpl};
use std::sync::Arc;

pub struct UserApiKeyMatcher {
    users: Arc<UsersImpl>,
}

impl UserApiKeyMatcher {
    pub fn new(users: Arc<UsersImpl>) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            users: Arc::clone(&users),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.users.find_by_api_key(api_key).map(|user| user.id)
    }
}
