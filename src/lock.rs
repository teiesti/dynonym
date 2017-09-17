use errors::*;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;

pub struct Lock {
    path: PathBuf,
}

impl Lock {
    pub fn new(path: PathBuf) -> Result<Self> {
        // Create a lock file
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .chain_err(|| "Could not create a lock file")?;

        // Write the process id into the lock file
        let pid = format!(
            "{}\n",
            unsafe { ::libc::getpid() }
        );
        file.write_all(pid.as_bytes())
            .chain_err(|| "Error while writing lock file")?;

        // Create struct
        let lock = Lock { path };
        Ok(lock)
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        // Remove the lock file
        remove_file(&self.path)
            .expect("Could not remove lock file");  // Should never panic
    }
}
