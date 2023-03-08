use std::net::SocketAddr;
use serde::Deserialize;
use crate::envy_load;

/// the api configuration of Recover State Server.
#[derive(Default, Debug, Deserialize, Clone, PartialEq)]
pub struct ApiConfig{
    /// Port to which the API server is listening.
    pub server_http_port: u16,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        envy_load!("api", "RUNTIME_CONFIG_")
    }

    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new("0.0.0.0".parse().unwrap(), self.server_http_port)
    }
}