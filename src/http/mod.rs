pub mod errors;
pub mod routes;

pub fn serve() {
    ::rocket::ignite()
        .mount("/", routes![
            routes::dns::update,
            routes::ip,
        ])
        .catch(errors![
            errors::unauthorized,
        ])
        .launch();
}
