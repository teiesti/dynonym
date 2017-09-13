use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::status::Custom;

#[error(401)]
fn unauthorized<'r>(req: &Request) -> Response<'r> {
    Response::build_from(
        Custom(Status::Unauthorized, "401 Unauthorized")
            .respond_to(req)
            .unwrap()
    )
    .raw_header("WWW-Authenticate", "Basic realm=\"dynonym\"")
    .finalize()
}
