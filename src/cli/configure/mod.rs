pub mod default;
pub mod dns;
pub mod http;
pub mod users;

use errors::*;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("configure")
        .about("Makes changes to the configuration file")

        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommand(default::setup())
        .subcommand(    dns::setup())
        .subcommand(   http::setup())
        .subcommand(  users::setup())
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Match and execute a subcommand
    match args.subcommand() {
        ("default", Some(args)) => default::call(args),
        ("dns"    , Some(args)) =>     dns::call(args),
        ("http"   , Some(args)) =>    http::call(args),
        ("users"  , Some(args)) =>   users::call(args),
        _                       =>      unreachable!(),
    }
}
