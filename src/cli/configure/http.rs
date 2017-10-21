use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("http")
        .about("Makes changes to the HTTP configuration")

        .arg(
            Arg::with_name("socket")
                .long("socket")
                .value_name("ADDR")
                .help("Changes the socket address")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("workers")
                .long("workers")
                .value_name("NUMBER")
                .help("Changes the number of worker threads")
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
        config.http.socket = socket;
    }

    // Change number of workers, if requested
    if let Some(workers_str) = args.value_of("workers") {
        let workers = workers_str.parse()?; // TODO Chain the error!
        config.http.workers = workers;
    }

    // Store the config
    config.store(config_file)?;

    Ok(())
}
