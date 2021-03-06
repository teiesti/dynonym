//! Configuration file parsing
//!
//! `dynonym` is configured using a TOML-encoded configuration file called `dynonym.toml`. This
//! module implements routines for loading and storing that file, and for manipulating a
//! configuration that is held in memory.
//!
//! A [`Config`] is a configuration held in memory. It can be loaded from the filesystem and stored
//! there. It provides an interface for manipulating any parameter. [`Config`] does not exhibit any
//! parameter directly. In order to enhance neatness, parameters are grouped into categories which
//! are then represented by other structs within a whole "tree of structs". [`Config`] is the
//! tree's root.
//!
//! [`Config`]: struct.Config.html
//!
//! # Examples
//!
//! Have a look at these examples to learn how the API works:
//!
//! ## Create a default configuration
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

/// An entire configuration for `dynonym` held in memory.
///
/// `Config` represents an entire configuration for `dynonym` held in memory. It is made out of
/// parts that configure certain subsystem. A `Config` can be saved in a configuration file and
/// restored from there.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// The HTTP server configuration.
    pub http: Http,

    /// The DNS update client configuration.
    pub dns: Dns,

    /// The "set" of authorized users.
    pub users: Users,
}

impl Config {
    /// Loads a configuration from file given a path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Open the config file read-only
        let mut file = File::open(&path)
            .chain_err(|| ErrorKind::ConfigFileOpen(path.as_ref().to_owned()))?;

        // Read the config file
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .chain_err(|| ErrorKind::ConfigFileRead(path.as_ref().to_owned()))?;

        // Decode configuration
        let config: Config = toml::from_str(buf.as_str())
            .chain_err(|| ErrorKind::ConfigFileDecode(path.as_ref().to_owned()))?;

        Ok(config)
    }

    /// Stores a configuration into a file given a path.
    ///
    /// This method will truncate an existing file without asking!
    pub fn store<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        // Open the config file write-only, truncate it if it exists
        let mut file = File::create(&path)
            .chain_err(|| ErrorKind::ConfigFileCreate(path.as_ref().to_owned()))?;

        // Encode configuration
        let buf = toml::to_string(&self)
            .chain_err(|| ErrorKind::ConfigFileEncode(path.as_ref().to_owned()))?;

        // Write the config file
        file.write_all(buf.as_bytes())
            .chain_err(|| ErrorKind::ConfigFileWrite(path.as_ref().to_owned()))?;

        Ok(())
    }

    /// Provides immutable access to an authorized user given the name.
    pub fn user(&self, user: &str) -> Option<&User> {
        self.users.get(user)
    }

    /// Provides mutable access to an authorized user given the name.
    pub fn user_mut(&mut self, user: &str) -> Option<&mut User> {
        self.users.get_mut(user)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: Http {
                socket: "127.0.0.1:8053".parse().unwrap(),
                workers: 2 * ::num_cpus::get() as u16,
                // log_level: (),
            },
            dns: Dns {
                socket: "127.0.0.1:53".parse().unwrap(),
                ttl: 60 /*sec*/,
            },
            users: Users::new(),
        }
    }
}

/// A configuration for the HTTP server.
#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    /// The socket address (== IP address and port number).
    pub socket: SocketAddr,

    /// The number of worker threads spinned up.
    pub workers: u16,

    // pub log_level: (), // TODO Find a good type!
}

/// A configuration for the DNS update client.
#[derive(Debug, Deserialize, Serialize)]
pub struct Dns {
    /// The socket address (== IP address and port number).
    pub socket: SocketAddr,

    /// The time-to-live used for any request.
    pub ttl: u32,
}

/// A mapping from users (== names) to settings (== passwords and lists of domains the user is
/// authorized for).
#[derive(Debug, Deserialize, Serialize)]
pub struct Users(HashMap<String, User>);

impl Users {
    /// Creates a new, empty mapping.
    pub fn new() -> Self {
        Users(HashMap::new())
    }

    /// Adds new user with a given name and password and an empty list of authorized domains into
    /// the mapping. If the given name is already mapped, the value is replaced. In that case, the
    /// old value is returned.
    pub fn add<T: Into<String>>(&mut self, user: T, pw: &str) -> Option<User> {
        self.insert(user.into(), User::with_pw(pw))
    }
    /// Removes an existing mapping given the user's name. In case the name was mapped, the old
    /// value is returned.
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

/// A setting corresponding with a user.
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// The salted and cryptographically hashed password.
    pub pw: Hash,

    /// A list of domains the user is authorized for.
    pub domains: Domains,
}

impl User {
    /// Creates a new user setting with a given password and an empty list of authorized domains.
    pub fn with_pw(pw: &str) -> Self {
        Self {
            pw: pw.into(),
            domains: Domains::new(),
        }
    }
}

/// A set of domains a user is authorized for.
#[derive(Debug, Deserialize, Serialize)]
pub struct Domains(HashSet<Domain>);

impl Domains {
    /// Create a new, empty set of domains.
    pub fn new() -> Self {
        Domains(HashSet::new())
    }

    /// Adds a given domain to the set.
    pub fn add(&mut self, domain: Domain) -> bool {
        self.insert(domain)
    }

    /// Returns a given domain from the set.
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
