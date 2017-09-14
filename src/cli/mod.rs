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
        //.global_setting(AppSettings::SubcommandRequired)
        .global_setting(AppSettings::SubcommandRequiredElseHelp)
        //.global_setting(AppSettings::VersionlessSubcommands)

        .subcommand(dns::setup())
        .subcommand(serve::setup())
        .subcommand(user::setup())
}

pub fn call(_args: &ArgMatches) {
    unimplemented!()
}
