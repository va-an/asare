use domain::users::{CreateUserRequest, User};

use super::service::UserService;

pub struct UsersController {
    user_svc: UserService,
}

impl UsersController {
    pub fn new(user_svc: UserService) -> UsersController {
        UsersController { user_svc }
    }

    pub fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        self.user_svc.create(create_user_request)
    }
}
