use std::sync::Arc;

use crate::entities::users::Users;

pub struct UserApiKeyMatcher {
    users: Arc<Users>,
}

impl UserApiKeyMatcher {
    pub fn new(user_service: Arc<Users>) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            users: Arc::clone(&user_service),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.users.find_by_api_key(api_key).map(|user| user.id)
    }
}
