use clap::{App, Arg, ArgMatches};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub http_host: String,
    pub http_port: u16,
}

impl Config {
    pub fn load() -> Config {
        let args = Config::parse_args();
        let dotenv_path = args.value_of("config");

        Config::load_dotenv(dotenv_path);
        Config::load_envy()
    }

    fn parse_args() -> ArgMatches {
        App::new("asare-backend-rs")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .short('c')
                    .takes_value(true)
                    .help("Sets a path for .env file"),
            )
            .get_matches()
    }

    // TODO: to Result
    fn load_dotenv(dotenv_path: Option<&str>) {
        match dotenv_path {
            Some(config_path) => match dotenv::from_path(config_path) {
                Ok(_) => log::debug!("dotenv - loaded .env file from {}", config_path),
                Err(error) => {
                    log::error!("dotenv - error with load .env file from {}", config_path);
                    panic!("{:#?}", error)
                }
            },
            None => match dotenv() {
                Ok(_) => log::debug!("dotenv - loaded config"),
                Err(error) => {
                    log::error!("error loading .env file");
                    panic!("{:#?}", error)
                }
            },
        }
    }

    // TODO: to Result
    fn load_envy() -> Config {
        let config = match envy::from_env::<Config>() {
            Ok(config) => {
                log::info!("Loaded config: \n{:#?}", config);
                config
            }
            Err(error) => {
                log::error!("Error loading config: {}", error);
                panic!("{:#?}", error)
            }
        };

        config
    }
}
