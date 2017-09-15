use errors::*;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    users: BTreeSet<User>,
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
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize, Ord, PartialOrd)]
pub struct User {
    user: String,
    pw_hash: String,
    records: BTreeSet<String>,
}

impl User {
    pub fn new<S, T, U>(user: S, pw: T) -> Self where
        S: Into<String>,
        T: Into<U>,
        U: AsRef<str>
    {
        use bcrypt::{hash, DEFAULT_COST};
        Self {
            user:    user.into(),
            pw_hash: hash(pw.into().as_ref(), DEFAULT_COST).unwrap(),
            records: BTreeSet::new(),
        }
    }

    pub fn has_pw<S, T>(&self, pw: S) -> bool where
        S: Into<T>,
        T: AsRef<str>
    {
        use bcrypt::verify;
        verify(pw.into().as_ref(), &self.pw_hash).unwrap()
    }
}
