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

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}
