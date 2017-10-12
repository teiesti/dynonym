use config::Config;
use errors::*;
use lock::Lock;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Starts the server")

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
}

pub fn call(args: &ArgMatches) -> Result<()> {
    // Create a lock
    // Note: The lock is auto-released when _lock goes out of scope!
    let _lock = Lock::create(
        args
            .value_of("lock")
            .unwrap()
            .into()
    )?.handle_sigint()?;

    // Load config
    let config = Config::load(
        args
            .value_of("config")
            .unwrap()
    )?;

    // Start the server
    ::http::serve(config)
}
