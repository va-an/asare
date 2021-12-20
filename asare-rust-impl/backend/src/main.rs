mod config;

use backend::AsareApp;
use config::Config;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = Config::load();
    let app = AsareApp::new(config.http_port);

    app.run().await
}
