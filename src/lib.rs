#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

pub mod routes;

pub fn serve() {
    rocket::ignite()
        .mount("/", routes![
            routes::dns::update,
            routes::ip,
        ])
        .launch();
}
