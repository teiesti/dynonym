use config::Config;
use errors::*;

use clap::{App, Arg, ArgMatches, SubCommand};

pub fn setup<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Manually updates a dynamic DNS resource record")

        .arg(
            Arg::with_name("DOMAIN")
                .help("Specifies the domain name")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("ipv4")
                .short("4")
                .alias("ip4")
                .long("ipv4")
                .value_name("ADDR")
                .help("Specifies the IPv4 address")
                .takes_value(true)
                .required_unless("ipv6")
        )
        .arg(
            Arg::with_name("ipv6")
                .short("6")
                .alias("ip6")
                .long("ipv6")
                .value_name("ADDR")
                .help("Specifies the IPv6 address")
                .takes_value(true)
                .required_unless("ipv4")
        )
}

pub fn call(_args: &ArgMatches, _config: Config) -> Result<()> {
    unimplemented!()
}
