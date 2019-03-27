#![warn(
    // missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
)]

pub mod config;

pub fn main() {
    // TODO DEBUG
    let config = config::Config::load("dynonym.toml.example");
    println!("{:#?}", config)
}
