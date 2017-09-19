use errors::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::path::Path;
use toml;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub http: HttpSetting,
    pub dns: DnsSetting,
    pub users: HashMap<String, UserSetting>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
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

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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

    // TODO keep this method?
    pub fn user(&self, user: &str) -> Option<&UserSetting> {
        self.users.get(user)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HttpSetting {
    // TODO currently unused
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsSetting {
    pub address: SocketAddr,
    pub ttl: u32,
}

impl Default for DnsSetting {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:53".parse().unwrap(),
            ttl: 60 /*sec*/,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSetting {
    pub password: Hash,
    pub domains: HashSet<String>,
}

impl UserSetting {
    pub fn with_password(pw: &str) -> Self {
        Self {
            password: pw.into(),
            domains: HashSet::new(),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Hash(String);

impl Hash {
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
        config.users.insert("tobias".into(), UserSetting::with_password("1234"));
        assert!(config.users.len() == 1);
        assert!(config.user("tobias").is_some());
        assert!(config.user("sebastian").is_none());
    }

    #[test]
    fn config_add_two_users() {
        // Create config with one user
        let mut config = Config::default();
        config.users.insert("tobias".into(), UserSetting::with_password("1234"));

        // Add another user
        config.users.insert("sebastian".into(), UserSetting::with_password("4321"));
        assert!(config.users.len() == 2);
        assert!(config.user("tobias").is_some());
        assert!(config.user("sebastian").is_some());
        assert!(config.user("stolzmann").is_none());
    }

    #[test]
    fn config_add_existing_user() {
        // Create config with one user
        let mut config = Config::default();
        config.users.insert("tobias".into(), UserSetting::with_password("1234"));

        // Insert a user with the same name but different password
        let old = config.users.insert("tobias".into(), UserSetting::with_password("4321"));
        assert!(config.users.len() == 1);
        assert!(config.user("tobias").unwrap().password.is("4321"));
        assert!(old.unwrap().password.is("1234"));
    }

    #[test]
    fn config_file() {
        use tempfile::NamedTempFile;

        // Create config
        let mut config = Config::default();
        config.users.insert("tobias".into(), UserSetting::with_password("1234"));
        config.users.insert("sebastian".into(), UserSetting::with_password("4321"));

        // Save
        let file = NamedTempFile::new().unwrap();
        let path = file.path();
        config.save(path).unwrap();

        // Load
        let config = Config::load(path).unwrap();
        assert!(config.users.len() == 2);
        assert!(config.user("tobias").unwrap().password.is("1234"));
        assert!(config.user("sebastian").unwrap().password.is("4321"));
    }

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
