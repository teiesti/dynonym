#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]

extern crate bcrypt;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate hyper;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate tempfile;
extern crate toml;
extern crate rocket;

pub mod cli;
pub mod config;
pub mod errors;
pub mod http;

pub use cli::main;
