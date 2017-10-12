use config::Config;
use errors::*;

use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("default")
        .about("Creates a default configuration file")
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Optain a default config
    let config = Config::default();

    // Store the config
    config.store(
        args
            .value_of("config")
            .unwrap()
    )?;

    Ok(())
}
