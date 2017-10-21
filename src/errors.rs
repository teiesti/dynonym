//! Error types and handling

use types::Domain;

use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;

error_chain! {
    foreign_links {
        Ctrlc(::ctrlc::Error)                   #[doc = "Error when setting up SIGINT handler"];
        Dns(::trust_dns::error::ClientError)    #[doc = "Error during DNS operation"];
        Io(::std::io::Error)                    #[doc = "Error during IO"];
        ParseInt(::std::num::ParseIntError)     #[doc = "Error when parsing an integer"];
        ParseNetAddr(::std::net::AddrParseError)
            #[doc = "Error when parsing an IP or socket address"];
        RocketConfig(::rocket::config::ConfigError)
            #[doc = "Error when creating a Rocket configuration"];
        TomlDe(::toml::de::Error)               #[doc = "Error when deserializing TOML"];
        TomlSer(::toml::ser::Error)             #[doc = "Error when serializing TOML"];
    }

    errors {
        ConfigFileOpen(path: PathBuf) {
            description("Cannot open config file")
            display("Cannot open config file '{}'", path.display())
        }

        ConfigFileCreate(path: PathBuf) {
            description("Cannot create config file")
            display("Cannot create config file '{}'", path.display())
        }

        ConfigFileRead(path: PathBuf) {
            description("Cannot read config file")
            display("Cannot read config file '{}'", path.display())
        }

        ConfigFileWrite(path: PathBuf) {
            description("Cannot write config file")
            display("Cannot write config file '{}'", path.display())
        }

        ConfigFileDecode(path: PathBuf) {
            description("Cannot decode config file")
            display("Cannot decode config file '{}'", path.display())
        }

        ConfigFileEncode(path: PathBuf) {
            description("Cannot encode config file")
            display("Cannot encode config file '{}'", path.display())
        }

        DnsConnOpen(socket: SocketAddr) {
            description("Cannot open a connection to the DNS server")
            display("Cannot open a connection to the DNS server at '{}'", socket)
        }

        DnsDomainConvert(domain: Domain) {
            description("Cannot convert domain into Trust DNS format")
            display("Cannot convert domain '{}' into Trust DNS format", domain)
        }

        DnsRecordCreate {
            description("Cannot create resource record")
        }

        DnsRecordDelete {
            description("Cannot delete resource record")
        }

        DnsRecordQuery {
            description("Cannot query resource record")
        }

        DnsUpdate(domain: Domain, ip: IpAddr) {
            description("Cannot update domain with IP address")
            display("Cannot update domain '{}' with IP address '{}'", domain, ip)
        }

        HttpConfig {
            description("Invalid HTTP configuration")
        }

        LockFileCreate(path: PathBuf) {
            description("Cannot create lock file")
            display("Cannot create lock file '{}'", path.display())
        }

        LockFileRemove(path: PathBuf) {
            description("Cannot remove lock file")
            display("Cannot remove lock file '{}'", path.display())
        }

        LockFileWrite(path: PathBuf) {
            description("Cannot write lock file")
            display("Cannot write lock file '{}'", path.display())
        }

        LockFileSetupSigintHandler {
            description(
                "Cannot create a handler that removes the lock file when receiving a SIGINT")
        }
    }
}

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
