//! # dynonym - The API documentation
//!
//! Welcome to the `dynonym` API documentation!
//!
//! This document is a technical reference for developers using `dynonym` as library crate.
//! Since `dynonym` is mainly used as an application, you're probably more interested in a user
//! guide. In that case, please have a look at the [README][readme] and consider using
//! `dynonym --help`!
//!
//! [readme]: https://github.com/teiesti/dynonym
//!
//! ## Usage
//!
//! In order to use `dynonym` within your project, you need to add the following dependency into
//! your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dynonym = "0.1"
//! ```
//!
//! A simple application may look like this:
//!
//! ```no_run
//! extern crate dynonym;
//!
//! fn main() {
//!     dynonym::main()
//! }
//! ```
//!
//! (This is actually all you need to mimic `dynonym`'s behavior since every little bit is
//! implemented within the library.)
//!
//! ## Module structure
//!
//! At the top level, modules can be grouped as follows:
//!
//! * Modules that fulfill a certain task
//!     * Modules that provide a remote interface
//!         * [`http`]: Web server (incl. routes)
//!         * [`dns`]: Domain Name System update client (RFC 2136: "DNS UPDATE")
//!     * Modules that deal with the operating system
//!         * [`cli`]: Command-line argument parsing and instruction assembly
//!         * [`config`]: Configuration file parsing
//!         * [`lock`]: Lock file management
//! * Modules that provide general support
//!     * [`types`]: Shared types (e.g. for a domain name)
//!     * [`errors`]: Error types and handling
//!
//! [`cli`]: cli/index.html
//! [`config`]: config/index.html
//! [`dns`]: dns/index.html
//! [`http`]: http/index.html
//! [`errors`]: errors/index.html
//! [`types`]: types/index.html
//! [`lock`]: lock/index.html

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
