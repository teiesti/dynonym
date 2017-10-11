// `error_chain!` can recurse deeply
#![recursion_limit="128"]

extern crate bcrypt;
#[macro_use] extern crate clap;
extern crate ctrlc;
#[macro_use] extern crate error_chain;
extern crate libc;
extern crate num_cpus;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate tempfile;
extern crate toml;
extern crate yansi;

pub mod cli;
pub mod config;
pub mod dns;
pub mod errors;
pub mod http;
pub mod lock;
pub mod types;

pub use cli::main;
