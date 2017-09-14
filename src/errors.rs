use std;
use toml;

error_chain! {
    foreign_links {
        Io(std::io::Error)          #[doc = "Error during IO"];
        TomlDe(toml::de::Error)     #[doc = "Error when deserializing TOML"];
        TomlSer(toml::ser::Error)   #[doc = "Error when serializing TOML"];
    }

    errors {
        // own errors here
    }
}
