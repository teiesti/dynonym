use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Adds a user")

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

    // Check if the users exists
    let user = args.value_of("USER").unwrap();
    if config.user(user).is_some() {
        bail!(format!("A user named '{}' already exists", user));   // TODO Use proper error!
    }

    // Prompt for a password
    let pw = ::rpassword::prompt_password_stdout("Please enter a password: ")?;

    // Create the user
    config.users.add(user, &pw);

    // Store the config
    config.store(config_file)?;

    Ok(())
}
