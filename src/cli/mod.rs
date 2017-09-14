use clap::{App, AppSettings, SubCommand};

pub fn handle() {

    let app =
        App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())

            .global_setting(AppSettings::ColoredHelp)
            .global_setting(AppSettings::GlobalVersion)
            //.global_setting(AppSettings::SubcommandRequired)
            .global_setting(AppSettings::SubcommandRequiredElseHelp)
            //.global_setting(AppSettings::VersionlessSubcommands)


            .subcommand(
                SubCommand::with_name("dns")
                    .about("Manually manages dynamic DNS resource records")
            )

            .subcommand(
                SubCommand::with_name("serve")
                    .about("Starts the server")
            )

            .subcommand(
                SubCommand::with_name("user")
                    .about("Manages users allowed to use the web frontend")
            )
        ;

    let _app_m = app.get_matches();

}
