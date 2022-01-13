use crate::{
    rebalancer::{
        rebalancer_service::{RebalanceInput, Rebalancer, RebalancerImpl},
        routes::RebalanceOutput,
    },
    users::users_service::{CreateUserRequest, User, UsersService},
};

pub type PresenterType = Box<dyn Presenter + Send + Sync>;

pub trait Presenter {}

pub struct HttpApiPresenter;

impl Presenter for HttpApiPresenter {}

pub struct RebalancerController {
    pub presenter: PresenterType,
}

impl RebalancerController {
    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        RebalancerImpl::rebalance(input)
    }
}

pub struct UsersController {
    pub users_svc: UsersService,
    pub presenter: PresenterType,
}

impl UsersController {
    pub fn create(&self, create_user_request: &CreateUserRequest) -> Result<User, String> {
        self.users_svc.create(create_user_request)
    }
}
