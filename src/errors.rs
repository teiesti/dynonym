//! Error types and handling
//!
//! This module contains error types and routines for error handling. Most parts are
//! auto-generated with [error-chain].
//!
//! [error-chain]: ../../error_chain/index.html

use types::Domain;

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

error_chain! {
    foreign_links {
        Ctrlc(::ctrlc::Error)                   #[doc = "Error when setting up SIGINT handler."];
        Dns(::trust_dns::error::ClientError)    #[doc = "Error during DNS operation."];
        Io(::std::io::Error)                    #[doc = "Error during IO."];
        ParseInt(::std::num::ParseIntError)     #[doc = "Error when parsing an integer."];
        ParseNetAddr(::std::net::AddrParseError)
            #[doc = "Error when parsing an IP or socket address."];
        RocketConfig(::rocket::config::ConfigError)
            #[doc = "Error when creating a Rocket configuration."];
        TomlDe(::toml::de::Error)               #[doc = "Error when deserializing TOML."];
        TomlSer(::toml::ser::Error)             #[doc = "Error when serializing TOML."];
    }

    errors {
        /// Error when opening a configuration file.
        ConfigFileOpen(path: PathBuf) {
            description("Cannot open config file")
            display("Cannot open config file '{}'", path.display())
        }

        /// Error when creating a configuration file.
        ConfigFileCreate(path: PathBuf) {
            description("Cannot create config file")
            display("Cannot create config file '{}'", path.display())
        }

        /// Error when reading a configuration file.
        ConfigFileRead(path: PathBuf) {
            description("Cannot read config file")
            display("Cannot read config file '{}'", path.display())
        }

        /// Error when writing a configuration file.
        ConfigFileWrite(path: PathBuf) {
            description("Cannot write config file")
            display("Cannot write config file '{}'", path.display())
        }

        /// Error when decoding a configuration file.
        ConfigFileDecode(path: PathBuf) {
            description("Cannot decode config file")
            display("Cannot decode config file '{}'", path.display())
        }

        /// Error when encoding a configuration file.
        ConfigFileEncode(path: PathBuf) {
            description("Cannot encode config file")
            display("Cannot encode config file '{}'", path.display())
        }

        /// Error when opening a connection to a DNS server.
        DnsConnOpen(socket: SocketAddr) {
            description("Cannot open a connection to the DNS server")
            display("Cannot open a connection to the DNS server at '{}'", socket)
        }

        /// Error when converting a domain into the TRust DNS format.
        DnsDomainConvert(domain: Domain) {
            description("Cannot convert domain into TRust DNS format")
            display("Cannot convert domain '{}' into TRust DNS format", domain)
        }

        /// Error when creating a resource record.
        DnsRecordCreate {
            description("Cannot create resource record")
        }

        /// Error when deleting a resource record.
        DnsRecordDelete {
            description("Cannot delete resource record")
        }

        /// Error when querying a resource record.
        DnsRecordQuery {
            description("Cannot query resource record")
        }

        /// Error when updating a domain with an IP address.
        DnsUpdate(domain: Domain, ip: IpAddr) {
            description("Cannot update domain with IP address")
            display("Cannot update domain '{}' with IP address '{}'", domain, ip)
        }

        /// Error caused by an invalid HTTP configuration.
        HttpConfig {
            description("Invalid HTTP configuration")
        }

        /// Error when creating a lock file.
        LockFileCreate(path: PathBuf) {
            description("Cannot create lock file")
            display("Cannot create lock file '{}'", path.display())
        }

        /// Error when removing a lock file.
        LockFileRemove(path: PathBuf) {
            description("Cannot remove lock file")
            display("Cannot remove lock file '{}'", path.display())
        }

        /// Error when writing a lock file.
        LockFileWrite(path: PathBuf) {
            description("Cannot write lock file")
            display("Cannot write lock file '{}'", path.display())
        }

        /// Error when creating a handler that removes the lock file when receiving a SIGINT.
        LockFileSetupSigintHandler {
            description(
                "Cannot create a handler that removes the lock file when receiving a SIGINT")
        }
    }
}

/// Handles a given error.
///
/// This function handles a given error. That includes:
///
/// 1. Printing the error message.
/// 2. Printing the error message of every causing error, recursively.
/// 3. Printing a backtrace, if available. (Try to run with `RUST_BACKTRACE=1` if no backtrace is
///    available).
/// 4. Exiting the process.
///
/// This function is intended to be used once per application at the very top layer. It should
/// handle every occuring error.
pub fn handle(err: &Error) -> ! {
    use yansi::Paint;

    // Print the error
    eprintln!("{} {}", Paint::red("error:").bold(), err);

    // Print the causing errors
    for err in err.iter().skip(1) {
        eprintln!("{} {}", Paint::blue("caused by:").bold(), err);
    }

    // Print the backtrace
    // The backtrace is not always generated. Try to run with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = err.backtrace() {
        eprintln!("{} {:?}", Paint::cyan("backtrace:").bold(), backtrace);
    }

    ::std::process::exit(1);
}
