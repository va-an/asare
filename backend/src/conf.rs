use clap::{arg, crate_version, Command};
use serde::Deserialize;
use tokio::{fs, io::AsyncReadExt};

#[derive(Debug, Deserialize)]
pub struct Db {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_host: String,
    pub http_port: u16,
    pub db: Db,
}

impl Config {
    pub async fn load() -> Config {
        let arg_matches = Command::new("asare-backend-rs")
            .version(crate_version!())
            .args(&[arg!(-c --config [FILE] "Load config from file")])
            .get_matches();

        let default_config_path = "backend.conf".to_string();

        let config_path = arg_matches
            .get_one::<String>("config")
            .unwrap_or(&default_config_path);

        let mut config_file = fs::File::open(config_path)
            .await
            .unwrap_or_else(|_| panic!("can't open '{}'", config_path));

        let mut config_str = String::new();

        config_file
            .read_to_string(&mut config_str)
            .await
            .expect("can't read from 'backend.json'");

        let config = serde_json::from_str(&config_str)
            .unwrap_or_else(|_| panic!("can't load config from {}", config_path));

        log::info!("Loaded config: \n{:#?}", config);

        config
    }
}
