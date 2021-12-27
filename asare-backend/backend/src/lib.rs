mod app;
mod conf;
mod rebalancer;
mod users;

use std::collections::HashMap;

use app::{ActixHttpServer, AsareHttpServer};

pub use crate::conf::Config;
pub use crate::rebalancer::domain::{RebalanceInput, Rebalancer, RebalancerV1};

pub type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    config: Config,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        AsareApp { config }
    }

    pub async fn run(self) -> std::io::Result<()> {
        ActixHttpServer::run_http_server(self.config).await
    }
}
