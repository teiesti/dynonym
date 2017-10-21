use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("auth")
        .about("Authorizes a user to update a domain")

        .arg(
            Arg::with_name("USER")
                .help("Specifies the user")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("DOMAIN")
                .help("Specifies the domain name")
                .required(true)
                .index(2)
        )
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Load the config
    let config_file = args.value_of("config").unwrap();
    let mut config = Config::load(config_file)?;

    {
        // Find the user
        let user_str = args.value_of("USER").unwrap();
        let user = match config.user_mut(user_str) {
            Some(user) => user,
            None => bail!(format!("A user named '{}' does not exist", user_str)),
            // TODO Use proper error!
        };

        // Decode the domain
        let domain_str = args.value_of("DOMAIN").unwrap();
        let domain = domain_str.parse()?;

        // Authorize the user for the domain
        if !user.domains.add(domain) {
            bail!(format!(
                "The user '{}' is already authorized for the domain '{}'",
                user_str,
                domain_str,
            ));
            // TODO Use proper error!
        }
    }

    // Store the config
    config.store(config_file)?;

    Ok(())
}
