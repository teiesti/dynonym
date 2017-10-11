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

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}
