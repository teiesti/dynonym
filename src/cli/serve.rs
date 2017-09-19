use config::Config;
use errors::*;

use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Starts the server")
}

pub fn call(_args: &ArgMatches, config: Config) -> Result<()> {
    ::http::serve(config)
}
