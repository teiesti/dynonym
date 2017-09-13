use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

#[derive(Debug)]
pub struct User {
    debug: String,  // TODO remove
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let auth = req
            .headers()
            .get_one("Authorization");

        match auth {
            Some(_) => Outcome::Success(
                User {
                    debug: format!("{:?}", auth),
                }
            ),
            None => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
