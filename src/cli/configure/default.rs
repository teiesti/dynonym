use errors::*;

use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("default")
        .about("Creates a default configuration file")
}

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}
