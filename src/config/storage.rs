pub struct StorageConfig;

impl StorageConfig {
    pub fn connection_string(&self) -> String {
        "postgres://postgres:password@localhost:5432/s4s".into()
    }
}
