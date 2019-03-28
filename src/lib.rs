#![warn(
    // missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
)]

pub mod cli;
pub mod config;
pub mod error;
pub mod lock;

use crate::cli::Opt;
use crate::config::Config;
use crate::error::{Error, Log};
use crate::lock::Lock;

use structopt::StructOpt;

pub fn try_main() -> Result<(), Error> {
    // Parse command line arguments
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    // Parse configuration file
    let config = Config::load(opt.config)?;
    println!("{:#?}", config);

    // Create a lock file
    // Note: The lock is auto-released when _lock goes out of scope!
    let _lock = Lock::create(opt.lock)?.handle_sigint()?;

    // Start the server
    // TODO

    Ok(())
}

pub fn main() {
    if let Err(err) = try_main() {
        err.log().abort();
    }
}
