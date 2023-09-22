use clap::{arg, crate_version, Command};
use config::Config;
use serde::Deserialize;

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
pub struct AppConfig {
    pub http_host: String,
    pub http_port: u16,
    pub db: Db,
}

impl AppConfig {
    pub async fn load() -> AppConfig {
        let arg_matches = Command::new("asare-backend-rs")
            .version(crate_version!())
            .args(&[arg!(-c --config [FILE] "Load config from file")])
            .get_matches();

        let default_config_path = "backend.toml".to_string();

        let config_path = arg_matches
            .get_one::<String>("config")
            .unwrap_or(&default_config_path);

        let settings = Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .expect("can't build config");

        let app_config: AppConfig = settings
            .try_deserialize::<AppConfig>()
            .expect("can't parse config");

        log::info!("Loaded config: \n{:#?}", app_config);

        app_config
    }
}
