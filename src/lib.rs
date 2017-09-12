#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

pub fn serve() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
