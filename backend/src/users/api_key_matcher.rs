use std::sync::Arc;

use super::users_service::UsersService;

pub struct UserApiKeyMatcher {
    users: UsersService,
}

impl UserApiKeyMatcher {
    pub fn new(users: UsersService) -> UserApiKeyMatcher {
        UserApiKeyMatcher {
            users: Arc::clone(&users),
        }
    }

    pub fn find_user_id(&self, api_key: &str) -> Option<i32> {
        self.users.find_by_api_key(api_key).map(|user| user.id)
    }
}
