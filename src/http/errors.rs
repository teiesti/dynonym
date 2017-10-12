use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::status::Custom;

#[error(400)]
pub fn bad_request() -> &'static str {
    "400 Bad Request"
}

#[error(401)]
pub fn unauthorized<'r>(req: &Request) -> Response<'r> {
    Response::build_from(
        Custom(Status::Unauthorized, "401 Unauthorized")
            .respond_to(req)
            .unwrap()
    )
    .raw_header("WWW-Authenticate", "Basic realm=\"dynonym\"")
    .finalize()
}

#[error(403)]
pub fn forbidden() -> &'static str {
    "403 Forbidden"
}

#[error(404)]
pub fn not_found() -> &'static str {
    "404 Not Found"
}

#[error(500)]
pub fn internal_server_error() -> &'static str {
    "500 Internal Server Error"
}

#[error(501)]
pub fn not_implemented() -> &'static str {
    "501 Not Implemented"
}
