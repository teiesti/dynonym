//! Web server (incl. routes)

pub mod errors;
pub mod routes;

use config::Config;
use errors::*;

use rocket::config::Environment;

pub fn serve(config: Config) -> Result<()> {
    // TODO Remove as soon as the errors![] macro bugfix was included in Rocket!
    use rocket;

    // Assemble the Rocket configuration
    let rocket_config = ::rocket::Config
        ::build(Environment::Development)
        .address(format!("{}", config.http.socket.ip()))
        .port(config.http.socket.port())
        .workers(config.http.workers)
        .finalize()
        .chain_err(|| ErrorKind::HttpConfig)?;

    // Configure the HTTP server and start it
    ::rocket::custom(rocket_config, true)
        .mount("/", routes![
            // routes::dns::update,
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
        .launch();

    Ok(())
}
