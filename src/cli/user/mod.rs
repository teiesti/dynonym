use clap::{App, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("user")
        .about("Manages users allowed to use the web frontend")
}

pub fn call(_args: &ArgMatches) {
    unimplemented!()
}
