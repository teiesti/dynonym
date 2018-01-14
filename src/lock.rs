//! Lock file management

use errors::*;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;

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
            .chain_err(|| ErrorKind::LockFileCreate(path.clone()))?;

        // Write the process id into the lock file
        let pid = format!(
            "{}\n",
            unsafe { ::libc::getpid() }
        );
        file.write_all(pid.as_bytes())
            .chain_err(|| ErrorKind::LockFileWrite(path.clone()))?;

        // Create struct
        let lock = Lock { path };
        Ok(lock)
    }

    pub fn handle_sigint(self) -> Result<Self> {
        // Create a clone
        //
        // WARNING: Do not derive Clone for Lock in order to simplify this operation since it is
        // not a good idea to have a twin in the wild. The relation between a Lock and its lock
        // file should be 1-to-1 because a lock -- when going out of scope -- removes the lock
        // file. Having a clone will result in a runtime error as soon as the second lock goes out
        // of scope and tries to remove the -- no longer existing -- lock file.
        let twin = Lock { path: self.path.clone() };

        // Setup SIGINT handler
        // TODO Replace ctrlc by tokio-signal?!
        ::ctrlc::set_handler(move || {
            if let Err(err) = twin.release_ref() {
                handle(&err);
            }
            ::std::process::exit(0);
        })
            .chain_err(|| ErrorKind::LockFileSetupSigintHandler)?;

        // Return self
        Ok(self)
    }

    pub fn release(self) { }

    fn release_ref(&self) -> Result<()> {
        // Remove the lock file
        remove_file(&self.path)
            .chain_err(|| ErrorKind::LockFileRemove(self.path.clone()))?;

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
