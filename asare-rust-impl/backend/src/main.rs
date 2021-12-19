use backend::AsareApp;
use dotenv::dotenv;
use env_logger::Env;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    http_port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let config = match envy::from_env::<Config>() {
        Ok(config) => {
            log::info!("Loaded config: \n{:#?}", config);
            config
        }
        Err(error) => panic!("{:#?}", error),
    };

    AsareApp::new(config.http_port).run().await
}
