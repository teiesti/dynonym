use config::Config;
use types::Domain;

use hyper::header::{Authorization, Basic, Header};
use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::response::Failure;
use rocket::request::{self, FromRequest, Request};
use std::net::{Ipv4Addr, Ipv6Addr};

#[get("/dns/update?<update>")]
pub fn update(config: State<Config>, creds: Credentials, update: Update) -> Result<(), Failure> {
    // Verify the credentials
    let user = config.user(&creds.user).ok_or(Failure(Status::Unauthorized))?;
    if !user.pw.is(&creds.pw) {
        return Err(Failure(Status::Unauthorized));
    }

    // Check the authorization
    user.domains.get(&update.domain).ok_or(Failure(Status::Forbidden))?;

    // Perform the update
    let dns = ::dns::Updater::new(config.dns.socket, config.dns.ttl).unwrap();  // TODO rm unwrap!
    if let Some(ipv4) = update.ipv4 {
        dns.update(update.domain.clone(), ipv4.into()).unwrap();    // TODO rm unwrap!
    }
    if let Some(ipv6) = update.ipv6 {
        dns.update(update.domain, ipv6.into()).unwrap();    // TODO rm unwrap!
    }

    Ok(())
}

#[derive(Debug)]
pub struct Credentials {
    user: String,
    pw: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Credentials {
    type Error = ();    // TODO May use a better error!

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Extract base64-encoded HTTP Authorization header, if existent
        if let Some(auth) = req
            .headers()
            .get_one("Authorization")
        {
            // Decode HTTP Authorization header
            if let
                Ok(
                    Authorization(
                        Basic {
                            username: user,
                            password: Some(pw)
                        }
                    )
                )
            = Authorization::<Basic>::parse_header(&auth.into()) {
                return Outcome::Success(
                    Self { user, pw }
                )
            }
        }

        // If anything goes wrong, fail with 401 Unauthorized
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[derive(Debug, FromForm)]
pub struct Update {
    domain: Domain,
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}
