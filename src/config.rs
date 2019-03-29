use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub http: Http,
    pub dns: Dns,
    pub users: Users,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Open the config file read-only
        let mut file = File::open(&path)
            .map_err(|_| Error::Open(path.as_ref().to_owned()))?;

        // Read the config file
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|_| Error::Read(path.as_ref().to_owned()))?;

        // Decode configuration
        let config: Config = toml::from_str(buf.as_str())
            .map_err(|_| Error::Decode(path.as_ref().to_owned()))?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub socket: SocketAddr,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dns {
    pub socket: SocketAddr,
    pub ttl: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users(HashMap<String, User>);

impl Users {
    pub fn authenticate(&self, user: &str, pw: &str) -> bool {
        self.get(user).map(|user| user.pw.is(pw)).unwrap_or(false)
    }

    pub fn authorize(&self, user: &str, domain: &str) -> bool {
        self.get(user).map(|user| user.domains.contains(domain)).unwrap_or(false)
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
    pub domains: HashSet<String>,
}

/// A salted and cryptographically hashed string.
///
/// A `Hash` represents a string that was salted and cryptographically hashed using the bcrypt
/// algorithm. The salt is stored alongside the hash. A `Hash` is well suited to store encrypted
/// passwords.
///
/// Since `Hash` implements `From<&str>`, the preferred method to obtain a `Hash` is to convert a
/// string slice using `Into<Hash>` as shown in the example below.
///
/// A `Hash` can be compared to a given string slice (== verified) with the method [`is`].
///
/// Because of different, randomly chosen salts, two hashes are (almost) never equal, even if
/// obtained from the exact same plain text.
///
/// [`is`]: #method.is
///
/// # Example
///
/// ```
/// use dynonym::config::Hash;
///
/// // Create
/// let h: Hash = "foo".into();
///
/// // Verify
/// assert!( h.is("foo"));
/// assert!(!h.is("bar"));
/// ```
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Hash(String);

impl Hash {
    /// Verifies whether `self` is a hashed version of the given string slice.
    pub fn is(&self, plain: &str) -> bool {
        use bcrypt::verify;
        verify(plain, &self.0).unwrap()
    }
}

impl<'a> From<&'a str> for Hash {
    fn from(plain: &'a str) -> Self {
        use bcrypt::{hash, DEFAULT_COST};
        let hash = hash(plain, DEFAULT_COST).unwrap();
        Hash(hash)
    }
}

#[derive(Debug)]
pub enum Error {
    Open(PathBuf),
    Read(PathBuf),
    Decode(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Open(path) => write!(f, "cannot open `{}`", path.display()),
            Error::Read(path) => write!(f, "cannot read `{}`", path.display()),
            Error::Decode(path) => write!(f, "cannot decode `{}`", path.display()),
        }
    }
}

impl StdError for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_eq() {
        let h: Hash = "foo".into();
        assert!(h.is("foo"));
    }

    #[test]
    fn hash_ne() {
        let h: Hash = "foo".into();
        assert!(!h.is("bar"));
    }

    #[test]
    fn hash_salt() {
        let h1: Hash = "foo".into();
        let h2: Hash = "foo".into();
        assert!(h1 != h2);  // different salts!
    }
}
