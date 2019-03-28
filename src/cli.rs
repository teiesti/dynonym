use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "structopt::clap::AppSettings::GlobalVersion"))]
pub struct Opt {
    #[structopt(
        short = "c",
        alias = "conf",
        long = "config",
        help = "Sets the path to the configuration file",
        default_value = "dynonym.toml",
        parse(from_os_str),
    )]
    pub config: PathBuf,

    #[structopt(
        short = "l",
        long = "lock",
        help = "Sets the path to the lock file",
        default_value = "dynonym.lock",
        parse(from_os_str),
    )]
    pub lock: PathBuf,
}
