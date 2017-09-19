use hyper::header::{Authorization, Basic, Header};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use std::net::{Ipv4Addr, Ipv6Addr};

#[get("/dns/update?<update>")]
pub fn update(creds: Credentials, update: Update) -> String {
    // TODO use the data
    format!("{:?}\n{:?}", creds, update)
}

#[derive(Debug)]
pub struct Credentials {
    user: String,
    password: String
}

impl<'a, 'r> FromRequest<'a, 'r> for Credentials {
    type Error = ();

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
                            password: Some(password)
                        }
                    )
                )
            = Authorization::<Basic>::parse_header(&auth.into()) {
                return Outcome::Success(
                    Self { user, password }
                )
            }
        }

        // If anything goes wrong, fail with 401 Unauthorized
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[derive(Debug, FromForm)]
pub struct Update {
    domain: String,
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}
