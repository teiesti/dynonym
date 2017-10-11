pub mod add;
pub mod auth;
pub mod pw;
pub mod rm;

use errors::*;

use clap::{App, AppSettings, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("users")
        .about("Manages the users allowed to use the web frontend")

        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommand( add::setup())
        .subcommand(auth::setup())
        .subcommand(  pw::setup())
        .subcommand(  rm::setup())
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Match and execute a subcommand
    match args.subcommand() {
        ("add" , Some(args)) =>  add::call(args),
        ("auth", Some(args)) => auth::call(args),
        ("pw"  , Some(args)) =>   pw::call(args),
        ("rm"  , Some(args)) =>   rm::call(args),
        _                    =>   unreachable!(),
    }
}
