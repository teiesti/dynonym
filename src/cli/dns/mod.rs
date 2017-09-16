pub mod update;

use errors::*;
use state::StateBuilder;

use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("dns")
        .about("Manually manages dynamic DNS resource records")

        .subcommand(update::setup())
}

pub fn call(_args: &ArgMatches, _state: StateBuilder) -> Result<()> {
    unimplemented!()
}
