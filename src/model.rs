use hyper::header::{Authorization, Basic, Header};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User {
    user: String,
    password: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
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
                    User { user, password }
                )
            }
        }

        // If anything goes wrong, fail with 401 Unauthorized
        Outcome::Failure((Status::Unauthorized, ()))
    }
}
