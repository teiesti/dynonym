#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]   // `error_chain!` can recurse deeply

extern crate bcrypt;
#[macro_use] extern crate clap;
extern crate ctrlc;
#[macro_use] extern crate error_chain;
extern crate hyper;
extern crate libc;
extern crate num_cpus;
extern crate rocket;
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
