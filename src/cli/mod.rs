pub mod dns;
pub mod serve;
pub mod user;

use clap::{App, AppSettings, ArgMatches};

pub fn run() {
    let args = setup().get_matches();
    call(&args);
}

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())

        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::GlobalVersion)
        //.global_setting(AppSettings::VersionlessSubcommands)

        //.setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommand(dns::setup())
        .subcommand(serve::setup())
        .subcommand(user::setup())
}

pub fn call(args: &ArgMatches) {
    match args.subcommand() {
        ("dns",     Some(args)) =>   dns::call(args),
        ("serve",   Some(args)) => serve::call(args),
        ("user",    Some(args)) =>  user::call(args),
        _                       =>    unreachable!(),
    }
}
