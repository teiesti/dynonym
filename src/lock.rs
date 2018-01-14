//! Lock file management
//!
//! When dealing with server software, it is often necessary to make sure that only one instance
//! is running at a time. Lock files are a common way to do that. This module provides a
//! convenient abstraction over low-level commands for lock file management. All magic is done
//! inside of [`Lock`], an exclusive lock file handle.
//!
//! [`Lock`]: struct.Lock.html

use errors::*;

use std::fs::{OpenOptions, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;

/// A `Lock` is the exclusive handle representing an existing lock file.
///
/// When creating a `Lock`, a lock file is auto-created and stored at the given path. The current
/// process id (PID) is written into the lock file. When a `Lock` is dropped (== goes
/// out-of-scope), the lock file is removed.
///
/// A `Lock` does never exist without its lock file. Neither does a lock file exist without its
/// handle, except for one occasion: When the process receives a SIGINT, the destructor will not
/// run and the lock file remains in the file system while the process is stopped. It is possible
/// to mitigate this behavior with a SIGINT handler that catches the signal, removes the lock file
/// and then stops the process. Such a handler can be set up using [`handle_sigint`].
///
/// A `Lock` is exclusive: It is impossible to clone or create a `Lock` that refers to the same
/// lock file. The reason is that dropping a `Lock` referencing a lock file that has already been
/// removed, would cause a runtime error. If you need more than one handle consider using a smart
/// pointer.
///
/// [`handle_sigint`]: #method.handle_sigint
///
/// # Example
/// ```no_run
/// # use dynonym::lock::Lock;
/// let path = "dynonym.lock".into();
/// let lock = Lock::create(path).unwrap()
///     .handle_sigint().unwrap();  // optional
///
/// // some code
///
/// lock.release();                 // optional
/// ```
pub struct Lock {
    path: PathBuf,
}

impl Lock {
    /// Creates a new `Lock`. This includes
    ///
    /// 1. creating a new lock file at the given path and
    /// 2. saving the current process id (PID) into the file.
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

    /// Creates a SIGINT handler for the given `Lock` that makes sure the lock file is removed even
    /// if the process is interrupted.
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

    /// Releases the given `Lock` and removes the corresponding lock file.
    ///
    /// It most cases it is not necessary to explicitly call this function since the lock file is
    /// auto-removed as soon as the `Lock` is dropped.
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
