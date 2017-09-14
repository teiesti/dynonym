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
            errors::bad_request,
            errors::unauthorized,
            errors::forbidden,
            errors::not_found,
            errors::internal_server_error,
            errors::not_implemented,
        ])
        .launch();
}
