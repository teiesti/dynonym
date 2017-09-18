use errors::*;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Lock {
    path: PathBuf,
}

impl Lock {
    pub fn create(path: PathBuf) -> Result<Self> {
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

    pub fn handle_sigint(self) -> Result<Self> {
        // Create a clone
        let twin = self.clone();

        // Setup SIGINT handler
        ::ctrlc::set_handler(move || {
            if let Err(err) = twin.release_ref() {
                handle(&err);
            }
            ::std::process::exit(0);
        })
            .chain_err(|| "Could not setup SIGINT handler")?;

        // Return self
        Ok(self)
    }

    pub fn release(self) { }

    fn release_ref(&self) -> Result<()> {
        // Remove the lock file
        remove_file(&self.path)
            .chain_err(|| "Could not remove lock file")?;

        Ok(())
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if let Err(err) = self.release_ref() {
            handle(&err);
        }
    }
}
