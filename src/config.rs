//! Configuration file parsing
//!
//! This module provides configuration file parsing for dynonym. It implements the serialization
//! and deserialization of the `dynonym.toml`. Any parameter is exposed within a syntax-sugared
//! "tree of structs" with [`Config`] being the root.
//!
//! Consider the following examples for the most common use cases:
//!
//! [`Config`]: struct.Config
//!
//! # Examples
//!
//! ## Get a default configuration
//! ```
//! # use dynonym::config::Config;
//! # #[allow(unused_variables)]
//! let config = Config::default();
//! ```
//!
//! ## Change basic parameters
//! ```
//! # use dynonym::config::Config;
//! # let mut config = Config::default();
//! config.http.socket = "127.0.0.1::8080".parse().unwrap();
//! config.http.workers = 4;
//! # // TODO config.http.log_level = ();
//!
//! config.dns.socket = "127.0.0.1::53".parse().unwrap();
//! config.dns.ttl = 60 /*sec*/;
//! ```
//!
//! ## Add and remove a user
//! ```
//! # use dynonym::config::Config;
//! # let mut config = Config::default();
//! config.users.add("tobias", "s3cr3t");
//! config.users.rm("tobias");
//! ```
//!
//! ## Change and verify a user's password
//! ```
//! # use dynonym::config::Config;
//! # let mut config = Config::default();
//! # config.users.add("tobias", "o1d_s3cr3t");
//! // Note: The user must exists!
//! config.user("tobias").unwrap().pw = "s3cr3t".into();
//! assert!(config.user("tobias").unwrap().pw.is("s3cr3t"));
//! ```
//!
//! ## Grant and revoke a user's authorization to update a domain
//! ```
//! # use dynonym::config::Config;
//! # let mut config = Config::default();
//! # config.users.add("tobias", "s3cr3t");
//! config.user("tobias").unwrap().domains.add("example.org");
//! config.user("tobias").unwrap().domains.rm("example.org");
//! ```
//!
//! ## Load from and store into a configuration file
//! ```no_run
//! # use dynonym::config::Config;
//! # use dynonym::errors::Error;
//! # fn magic() -> Result<(), Error> {
//! let config = Config::load("dynonym.toml")?;
//! config.store("dynonym.toml")?;
//! # Ok(()) }
//! ```

use errors::*;
use types::{Domain, Hash};

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub http: HttpConfig,
    pub dns: DnsConfig,
    pub users: HashMap<String, UserConfig>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        unimplemented!()
    }

    pub fn store<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        unimplemented!()
    }

    pub fn user(&mut self, user: &str) -> Option<&mut UserConfig> {
        unimplemented!()
    }
}

impl Default for Config {
    fn default() -> Self {
        unimplemented!()
    }
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
