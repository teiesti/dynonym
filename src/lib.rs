#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate clap;
extern crate hyper;
extern crate rocket;

pub mod cli;
pub mod http;
