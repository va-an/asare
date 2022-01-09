use std::sync::Arc;

use crate::UserService;

pub struct UserApiKeyMatcher {
    user_service: Arc<UserService>,
}

impl UserApiKeyMatcher {
    pub fn new(user_service: Arc<UserService>) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            user_service: Arc::clone(&user_service),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.user_service
            .find_by_api_key(api_key)
            .map(|user| user.id)
    }
}
