use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    str::FromStr,
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    host: String,
    port: u16,
}

impl AppConfig {
    pub fn address(&self) -> Result<SocketAddr, AddrParseError> {
        let ip_addr = IpAddr::from_str(&self.host)?;
        Ok(SocketAddr::from((ip_addr, self.port)))
    }
}
