use errors::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct UserSetting {
    pub password: Hash,
    pub records: HashSet<String>,
}

impl UserSetting {
    pub fn with_password(pw: &str) -> Self {
        Self {
            password: pw.into(),
            records: HashSet::new(),
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
