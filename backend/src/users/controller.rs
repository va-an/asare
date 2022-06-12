use domain::User;

use super::users_service::{CreateUserRequest, UsersService};

pub struct UsersController {
    users_svc: UsersService,
}

impl UsersController {
    pub fn new(users_svc: UsersService) -> UsersController {
        UsersController { users_svc }
    }

    pub fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        self.users_svc.create(create_user_request)
    }
}
