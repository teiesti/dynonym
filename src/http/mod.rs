pub mod errors;
pub mod routes;

pub fn serve() {
    ::rocket::ignite()
        .mount("/", routes![
            routes::dns::update,
            routes::ip,
            routes::port,
            routes::socket,
        ])
        .catch(errors![
            errors::unauthorized,
        ])
        .launch();
}
