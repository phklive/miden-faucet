use serde::Deserialize;
use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
    vec,
};

// FAUCET CONFIG
// ================================================================================================

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FaucetTopLevelConfig {
    pub faucet: FaucetConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FaucetConfig {
    pub endpoint: Endpoint,
    pub database_filepath: String,
}

// ENDPOINT
// ================================================================================================

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Endpoint {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl ToSocketAddrs for Endpoint {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        (self.host.as_ref(), self.port).to_socket_addrs()
    }
}
