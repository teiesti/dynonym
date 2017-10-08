//! Configuration file parsing

use types::{Domain, Hash};

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub http: HttpConfig,
    pub dns: DnsConfig,
    pub users: HashMap<String, UserConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpConfig {
    pub socket: SocketAddr,
    pub workers: u16,
    pub log_level: (), // TODO Find a good type!
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsConfig {
    pub socket: SocketAddr,
    pub ttl: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub pw: Hash,
    pub domains: HashSet<Domain>,
}
