#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate hyper;
extern crate rocket;

pub mod cli;
pub mod errors;
pub mod http;
