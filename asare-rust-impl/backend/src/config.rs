use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub http_port: u16,
}

impl Config {
    pub fn load() -> Config {
        dotenv().ok();

        let config = match envy::from_env::<Config>() {
            Ok(config) => {
                log::info!("Loaded config: \n{:#?}", config);
                config
            }
            Err(error) => panic!("{:#?}", error),
        };

        config
    }
}
