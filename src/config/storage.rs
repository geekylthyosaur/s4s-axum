use serde::Deserialize;

#[derive(Deserialize)]
pub struct StorageConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database_name: String,
}

impl StorageConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
