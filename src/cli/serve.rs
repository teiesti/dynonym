use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Starts the server")

        .arg(
            Arg::with_name("lock")
                .short("l")
                .long("lock")
                .value_name("FILE")
                .help("Sets a custom lock file")
                .takes_value(true)
                .default_value(
                    concat!(crate_name!(), ".lock")
                )
        )
}

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}
