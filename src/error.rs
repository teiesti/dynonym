use crate::config;
use crate::lock;

use std::fmt;
use std::error::Error as StdError;
use yansi::Paint;

#[derive(Debug)]
pub enum Error {
    Config(config::Error),
    Lock(lock::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Config(err) => write!(f, "config: {}", err),
            Error::Lock(err) => write!(f, "lock: {}", err),
        }
    }
}

impl StdError for Error {}

impl From<config::Error> for Error {
    fn from(err: config::Error) -> Error {
        Error::Config(err)
    }
}

impl From<lock::Error> for Error {
    fn from(err: lock::Error) -> Error {
        Error::Lock(err)
    }
}

pub trait Log {
    fn log(self) -> Abort;
}

impl<E> Log for E
where
    E: StdError + Into<Error>
{
    fn log(self) -> Abort {
        // Convert the error
        let err: Error = self.into();

        // Print the error
        eprintln!("{} {}", Paint::red("error:").bold(), err);

        // Print the causing errors
        // TODO

        Abort {}
    }
}

pub struct Abort;

impl Abort {
    pub fn abort(self) -> ! {
        ::std::process::exit(1);
    }
}
