use clap::{App, AppSettings, SubCommand};

pub fn handle() {

    let app =
        App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())

            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::GlobalVersion)
            //.setting(AppSettings::SubcommandRequired)
            .setting(AppSettings::SubcommandRequiredElseHelp)
            //.setting(AppSettings::VersionlessSubcommands)

            .subcommand(
                SubCommand::with_name("dns")
                    .about("Manually manages dynamic DNS resource records")
                    .setting(AppSettings::ColoredHelp)
            )

            .subcommand(
                SubCommand::with_name("serve")
                    .about("Starts the server")
                    .setting(AppSettings::ColoredHelp)
            )

            .subcommand(
                SubCommand::with_name("user")
                    .about("Manages users allowed to use the web frontend")
                    .setting(AppSettings::ColoredHelp)
            )
        ;

    let _app_m = app.get_matches();

}
