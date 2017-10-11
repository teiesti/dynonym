//! Command-line parsing and instruction assembly

pub mod configure;
pub mod serve;

use errors::*;

use clap::{App, AppSettings, Arg, ArgMatches};

pub fn main() {
    // Parse arguments
    let args = setup().get_matches();

    // Run command
    let result = call(&args);

    // Handle error, if necessary
    if let Err(err) = result {
        handle(&err);
    }
}

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())

        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::GlobalVersion)
        .global_setting(AppSettings::DisableHelpSubcommand)

        .setting(AppSettings::SubcommandRequiredElseHelp)

        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .alias("conf")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value(
                    concat!(crate_name!(), ".conf")
                )
        )
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

        .subcommand(configure::setup())
        .subcommand(    serve::setup())
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Match and execute a subcommand
    match args.subcommand() {
        ("configure", Some(args)) => configure::call(args),
        ("serve",     Some(args)) =>     serve::call(args),
        _                         =>        unreachable!(),
    }
}
