#![feature(custom_derive, plugin)]
#![plugin(rocket_codegen)]

extern crate hyper;
extern crate rocket;

pub mod http;
pub mod model;
