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
//! config.http.socket = "127.0.0.1:8053".parse().unwrap();
//! config.http.workers = 4;
//! # // TODO config.http.log_level = ();
//!
//! config.dns.socket = "127.0.0.1:53".parse().unwrap();
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
//! config.user_mut("tobias").unwrap().pw = "s3cr3t".into();
//! assert!(config.user("tobias").unwrap().pw.is("s3cr3t"));
//! ```
//!
//! ## Grant and revoke a user's authorization to update a domain
//! ```
//! # use dynonym::config::Config;
//! # let mut config = Config::default();
//! # config.users.add("tobias", "s3cr3t");
//! config.user_mut("tobias").unwrap().domains.add("example.org".parse().unwrap());
//! config.user_mut("tobias").unwrap().domains.rm(&"example.org".parse().unwrap());
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
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub http: Http,
    pub dns: Dns,
    pub users: Users,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        // TODO Use better error types!

        // Open the config file read-only
        let mut file = File::open(path)
            .chain_err(|| "Unable to open config file")?;

        // Read the config file
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .chain_err(|| "Error while reading config file")?;

        // Decode configuration
        let config: Config = toml::from_str(buf.as_str())
            .chain_err(|| "Could not decode config file")?;

        Ok(config)
    }

    pub fn store<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // TODO Use better error types!

        // Open the config file write-only, truncate it if it exists
        let mut file = File::create(path)
            .chain_err(|| "Could not open config file")?;

        // Encode configuration
        let buf = toml::to_string(&self)
            .chain_err(|| "Could not encode config file")?;

        // Write the config file
        file.write_all(buf.as_bytes())
            .chain_err(|| "Error while writing config file")?;

        Ok(())
    }

    pub fn user(&self, user: &str) -> Option<&User> {
        self.users.get(user)
    }

    pub fn user_mut(&mut self, user: &str) -> Option<&mut User> {
        self.users.get_mut(user)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: Http {
                socket: "127.0.0.1:8053".parse().unwrap(),
                workers: 4, // TODO Use 2*num_cpu!
                log_level: (),
            },
            dns: Dns {
                socket: "127.0.0.1:53".parse().unwrap(),
                ttl: 60 /*sec*/,
            },
            users: Users::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub socket: SocketAddr,
    pub workers: u16,
    pub log_level: (), // TODO Find a good type!
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dns {
    pub socket: SocketAddr,
    pub ttl: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users(HashMap<String, User>);

impl Users {
    pub fn new() -> Self {
        Users(HashMap::new())
    }

    pub fn add<T: Into<String>>(&mut self, user: T, pw: &str) -> Option<User> {
        self.insert(user.into(), User::with_pw(pw))
    }

    pub fn rm(&mut self, user: &str) -> Option<User> {
        self.remove(user)
    }
}

impl Deref for Users {
    type Target = HashMap<String, User>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub pw: Hash,
    pub domains: Domains,
}

impl User {
    pub fn with_pw(pw: &str) -> Self {
        Self {
            pw: pw.into(),
            domains: Domains::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Domains(HashSet<Domain>);

impl Domains {
    pub fn new() -> Self {
        Domains(HashSet::new())
    }

    pub fn add(&mut self, domain: Domain) -> bool {
        self.insert(domain)
    }

    pub fn rm(&mut self, domain: &Domain) -> bool {
        self.remove(domain)
    }
}

impl Deref for Domains {
    type Target = HashSet<Domain>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Domains {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default() {
        let config = Config::default();
        assert!(config.users.is_empty());
        // TODO test other fields
    }

    #[test]
    fn config_add_one_user() {
        // Create an empty config
        let mut config = Config::default();
        assert!(config.users.is_empty());
        assert!(config.user("tobias").is_none());

        // Add a user
        config.users.add("tobias", "1234");
        assert!(config.users.len() == 1);
        assert!(config.user("tobias").is_some());
        assert!(config.user("sebastian").is_none());
    }

    #[test]
    fn config_add_two_users() {
        // Create config with one user
        let mut config = Config::default();
        config.users.add("tobias", "1234");

        // Add another user
        config.users.add("sebastian", "4321");
        assert!(config.users.len() == 2);
        assert!(config.user("tobias").is_some());
        assert!(config.user("sebastian").is_some());
        assert!(config.user("stolzmann").is_none());
    }

    #[test]
    fn config_add_existing_user() {
        // Create config with one user
        let mut config = Config::default();
        config.users.add("tobias", "1234");

        // Insert a user with the same name but different password
        let old = config.users.add("tobias", "4321");
        assert!(config.users.len() == 1);
        assert!(config.user("tobias").unwrap().pw.is("4321"));
        assert!(old.unwrap().pw.is("1234"));
    }

    #[test]
    fn config_file() {
        use tempfile::NamedTempFile;

        // Create config
        let mut config = Config::default();
        config.users.add("tobias", "1234");
        config.users.add("sebastian", "4321");

        // Save
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        config.store(path).unwrap();

        // Load
        let config = Config::load(path).unwrap();
        assert!(config.users.len() == 2);
        assert!(config.user("tobias").unwrap().pw.is("1234"));
        assert!(config.user("sebastian").unwrap().pw.is("4321"));
    }
}
