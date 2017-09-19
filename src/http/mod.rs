pub mod errors;
pub mod routes;

use config::Config;
use errors::*;

pub fn serve(config: Config) -> Result<()> {
    // Configure the provider managing DNS updates
    let dns = (); // TODO

    // Configure the HTTP server and start it
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
        .manage(config)
        .manage(dns)
        .launch();

    Ok(())
}
