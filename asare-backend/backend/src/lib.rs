mod app;
mod conf;
mod rebalancer;
mod users;

use app::{ActixHttpServer, AsareHttpServer};
use std::collections::HashMap;

pub use crate::conf::Config;
pub use crate::rebalancer::domain::{RebalanceInput, Rebalancer, RebalancerV1};
pub use crate::users::{repository::*, service::*};

pub type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    config: Config,
    user_service: UserService,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        let user_repo = UserRepoInMemory::new();
        let user_service = UserService::new(user_repo);

        AsareApp {
            config,
            user_service,
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        ActixHttpServer::run_http_server(self).await
    }
}
