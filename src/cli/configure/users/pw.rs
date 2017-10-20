use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("pw")
        .about("Changes a user's password")

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

    // Find the user
    let user = args.value_of("USER").unwrap();
    match config.user_mut(user) {
        Some(user) => {
            // Prompt for a password
            let pw = ::rpassword::prompt_password_stdout("Please enter a password: ")?;

            // Change the password
            user.pw = pw.as_str().into();
        },
        None => bail!(format!("A user named '{}' does not exist", user)), // TODO Use proper error!
    }

    // Store the config
    config.store(config_file)?;

    Ok(())
}
