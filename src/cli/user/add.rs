use errors::*;
use state::StateBuilder;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Adds a user")

        .arg(
            Arg::with_name("USER")
                .help("Specifies the user")
                .required(true)
                .index(1)
        )
}

pub fn call(_args: &ArgMatches, _state: StateBuilder) -> Result<()> {
    unimplemented!()
}
