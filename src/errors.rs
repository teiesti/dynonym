//! Error types and handling

use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(::std::io::Error)                    #[doc = "Error during IO"];
        TomlDe(::toml::de::Error)               #[doc = "Error when deserializing TOML"];
        TomlSer(::toml::ser::Error)             #[doc = "Error when serializing TOML"];
    }

    errors {
        ConfigFileOpen(path: PathBuf) {
            description("Cannot open config file")
            display("Cannot open config file '{}'", path.display())
        }

        ConfigFileCreate(path: PathBuf) {
            description("Cannot create config file")
            display("Cannot create config file '{}'", path.display())
        }

        ConfigFileRead(path: PathBuf) {
            description("Cannot read config file")
            display("Cannot read config file '{}'", path.display())
        }

        ConfigFileWrite(path: PathBuf) {
            description("Cannot write config file")
            display("Cannot write config file '{}'", path.display())
        }

        ConfigFileDecode(path: PathBuf) {
            description("Cannot decode config file")
            display("Cannot decode config file '{}'", path.display())
        }

        ConfigFileEncode(path: PathBuf) {
            description("Cannot encode config file")
            display("Cannot encode config file '{}'", path.display())
        }
    }
}

pub fn handle(err: &Error) -> ! {
    use std::io::Write;
    use yansi::Paint;

    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "{} {}", Paint::red("error:").bold(), err).expect(errmsg);

    for err in err.iter().skip(1) {
        writeln!(stderr, "{} {}", Paint::blue("caused by:").bold(), err).expect(errmsg);
    }

    // The backtrace is not always generated. Try to run with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = err.backtrace() {
        writeln!(stderr, "{} {:?}", Paint::cyan("backtrace:").bold(), backtrace).expect(errmsg);
    }

    ::std::process::exit(1);
}
