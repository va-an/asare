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
        let is_custom_path = args.value_of("config");

        // FIXME: error handling
        match is_custom_path {
            Some(config_path) => {
                dotenv::from_path(config_path);
                ()
            }
            None => {
                dotenv();
                ()
            }
        }

        let config = match envy::from_env::<Config>() {
            Ok(config) => {
                log::info!("Loaded config: \n{:#?}", config);
                config
            }
            Err(error) => panic!("{:#?}", error),
        };

        config
    }

    fn parse_args() -> ArgMatches<'static> {
        App::new("asare-backend-rs")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .short("c")
                    .takes_value(true)
                    .help("Sets a path for .env file"),
            )
            .get_matches()
    }
}
