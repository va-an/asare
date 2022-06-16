use std::sync::Arc;

use super::service::UserService;

pub struct UserApiKeyMatcher {
    user_svc: UserService,
}

impl UserApiKeyMatcher {
    pub fn new(users: UserService) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            user_svc: Arc::clone(&users),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.user_svc.find_by_api_key(api_key).map(|user| user.id)
    }
}
