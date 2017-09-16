pub mod add;
pub mod auth;
pub mod pw;
pub mod rm;

use errors::*;

use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("user")
        .about("Manages users allowed to use the web frontend")

        .subcommand(add::setup())
        .subcommand(auth::setup())
        .subcommand(pw::setup())
        .subcommand(rm::setup())
}

pub fn call(_args: &ArgMatches) -> Result<()> {
    unimplemented!()
}
