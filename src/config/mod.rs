use serde::Deserialize;

use self::{app::AppConfig, storage::StorageConfig};

mod app;
pub mod env;
pub mod routes;
mod storage;

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub storage: StorageConfig,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::File::from(
                std::env::current_dir()
                    .expect("Failed to determine the current directory.")
                    .join("config.yaml"),
            ))
            .build()?;

        config.try_deserialize::<Config>()
    }
}
