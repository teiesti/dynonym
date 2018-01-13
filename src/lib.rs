#![feature(
    custom_derive,
    plugin,
    try_from,
)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]   // `error_chain!` can recurse deeply
#![warn(
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
)]

extern crate bcrypt;
#[macro_use] extern crate clap;
extern crate ctrlc;
#[macro_use] extern crate error_chain;
extern crate hyper;
extern crate libc;
extern crate num_cpus;
extern crate rocket;
extern crate rpassword;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate tempfile;
extern crate toml;
extern crate trust_dns;
extern crate trust_dns_proto;
extern crate yansi;

pub mod cli;
pub mod config;
pub mod dns;
pub mod errors;
pub mod http;
pub mod lock;
pub mod types;

pub use cli::main;
