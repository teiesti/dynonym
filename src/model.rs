use rocket::request::{self, Request, FromRequest};

pub struct User;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(_req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        unimplemented!()
    }
}
