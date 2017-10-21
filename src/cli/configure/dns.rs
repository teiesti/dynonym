use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("dns")
        .about("Makes changes to the DNS configuration")

        .arg(
            Arg::with_name("socket")
                .long("socket")
                .value_name("ADDR")
                .help("Changes the socket address")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("ttl")
                .long("ttl")
                .value_name("SECONDS")
                .help("Changes the time to live (TTL)")
                .takes_value(true)
        )
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Load the config
    let config_file = args.value_of("config").unwrap();
    let mut config = Config::load(config_file)?;

    // Change socket, if requested
    if let Some(socket_str) = args.value_of("socket") {
        let socket = socket_str.parse()?;   // TODO Chain the error!
        config.dns.socket = socket;
    }

    // Change ttl, if requested
    if let Some(ttl_str) = args.value_of("ttl") {
        let ttl = ttl_str.parse()?;         // TODO Chain the error!
        config.dns.ttl = ttl;
    }

    // Store the config
    config.store(config_file)?;

    Ok(())
}
