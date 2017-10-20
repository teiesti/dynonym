use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("rm")
        .about("Removes a user")

        .arg(
            Arg::with_name("USER")
                .help("Specifies the user")
                .required(true)
                .index(1)
        )
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Load the config
    let config_file = args.value_of("config").unwrap();
    let mut config = Config::load(config_file)?;

    // Remove the user, if existing
    let user = args.value_of("USER").unwrap();
    if config.users.remove(user).is_none() {
        bail!(format!("A user named '{}' does not exist", user));   // TODO Use proper error!
    }

    // Store the config
    config.store(config_file)?;

    Ok(())
}
