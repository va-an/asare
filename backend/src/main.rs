mod app;
mod conf;
mod portfolios;
mod rebalancer;
mod users;

use crate::conf::AppConfig;
use app::AsareApp;
use env_logger::Env;

#[tokio::main]
async fn main() {
    // TODO: tracing
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = AppConfig::load().await;
    let app = AsareApp::new(config).await;

    app.run().await
}
