use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("auth")
        .about("Authorizes a user to update a resource record")

        .arg(
            Arg::with_name("USER")
                .help("Specifies the user")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("RECORD")
                .help("Specifies the record name (aka domain name)")
                .required(true)
                .index(2)
        )
}

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}