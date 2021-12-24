mod conf;
mod rebalancer;
mod routes;

pub use crate::conf::Config;
pub use crate::rebalancer::{RebalanceInput, Rebalancer, RebalancerV1};

use routes::{ActixHttpServer, AsareHttpServer};

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
