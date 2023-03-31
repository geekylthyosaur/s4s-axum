use self::{app::AppConfig, storage::StorageConfig};

mod app;
pub mod env;
pub mod routes;
mod storage;

pub struct Config {
    app: AppConfig,
    pub storage: StorageConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app: AppConfig,
            storage: StorageConfig,
        }
    }
}
