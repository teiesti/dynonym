use errors::*;
use state::StateBuilder;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Manually updates a dynamic DNS resource record")

        .arg(
            Arg::with_name("NAME")
                .help("Specifies the resource record name (aka domain name)")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("a")
                .short("4")
                .long("a")
                .value_name("A")
                .help("Specifies the A resource record (aka IPv4 address)")
                .takes_value(true)
                .required_unless("aaaa")
        )
        .arg(
            Arg::with_name("aaaa")
                .short("6")
                .long("aaaa")
                .value_name("AAAA")
                .help("Specifies the AAAA resource record (aka IPv6 address)")
                .takes_value(true)
                .required_unless("a")
        )
}

pub fn call(_args: &ArgMatches, _state: StateBuilder) -> Result<()> {
    unimplemented!()
}
