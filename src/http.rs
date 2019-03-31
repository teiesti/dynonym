use crate::config::Config;

use hyper::header;
use hyper::rt::{self, Future};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn_ok};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

pub fn serve(config: Arc<Config>) {
    let server_addr = config.http.socket;

    let router = make_service_fn(move |socket: &AddrStream| {
        let config = config.clone();
        let remote_addr = socket.remote_addr();
        service_fn_ok(route(config, remote_addr))
    });

    let server = Server::bind(&server_addr)
        .serve(router)
        // TODO graceful shutdown?
        .map_err(|e| eprintln!("server error: {}", e)); // TODO

    println!("Listening on http://{}", server_addr);

    rt::run(server);
}

pub fn route(
    config: Arc<Config>,
    addr: SocketAddr
) -> impl FnMut(Request<Body>) -> Response<Body> {
    move |request: Request<Body>| {
        if request.method() != Method::GET {
            return method_not_allowed(&[Method::GET])
        }

        let path = decode_path(&request);
        let query = decode_query(&request);
        let creds = decode_creds(&request);

        match path.as_slice() {
            &["ip"] => ip(addr),
            &["port"] => port(addr),
            &["socket"] => socket(addr),
            path_slice if path_slice.starts_with(&["rr"]) => {
                rr(&config, path, query, creds)
            },
            _ => not_found(),
        }
    }
}

pub fn decode_path(request: &Request<Body>) -> Vec<&str> {
    request.uri().path().split('/').skip(1).collect::<Vec<_>>()
}

pub fn decode_query(request: &Request<Body>) -> HashMap<&str, &str> {
    request.uri().query().map(|query| {
        query
            .split('&')
            .map(|pair| {
                let mut iter = pair.splitn(2, '=');
                (iter.next().unwrap_or(""), iter.next().unwrap_or(""))
            })
            .collect::<HashMap<_, _>>()
    }).unwrap_or_else(HashMap::new)
}

pub fn decode_creds(request: &Request<Body>) -> Option<(String, String)> {
    request.headers().get(header::AUTHORIZATION)
        .and_then(|x| x.to_str().ok())
        .and_then(|x| {
            let mut components = x.split_whitespace();
            if let Some("Basic") = components.next() {
                components.next()
            } else {
                None
            }
        })
        .and_then(|creds| base64::decode(creds).ok())
        .and_then(|creds| String::from_utf8(creds).ok())
        .and_then(|creds| {
            let mut components = creds.splitn(2, ':').map(|x| x.to_owned());
            match (components.next(), components.next()) {
                (Some(user), Some(pw)) => Some((user, pw)),
                _ => None,
            }
        })
}

pub fn ip(addr: SocketAddr) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}\n", addr.ip()).into())
        .unwrap()
}

pub fn port(addr: SocketAddr) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}\n", addr.port()).into())
        .unwrap()
}

pub fn socket(addr: SocketAddr) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}\n", addr).into())
        .unwrap()
}

pub fn rr(
    config: &Config,
    path: Vec<&str>,
    query: HashMap<&str, &str>,
    creds: Option<(String, String)>
) -> Response<Body> {
    if creds.is_none() {
        return unauthorized();
    }
    let (user, pw) = creds.unwrap();
    if !config.users.authenticate(&user, &pw) {
        return unauthorized();
    }

    let owner = match path.get(1) {
        Some(owner) => owner,
        None => return not_found(), // TODO List RR owners the client is authorized for?
    };
    if !config.users.authorize(&user, owner) {
        return forbidden()
    }

    let rtype = &match path.get(2) {
        Some(rtype) => rtype,
        None => return not_found(), // TODO List RR types with an existing record?
    }.to_uppercase();
    // TODO Restrict client to some RR types?

    let rdata = match query.get("rdata") {
        Some(rdata) => rdata,
        None => return bad_request(), // TODO Return RR that is stored at the moment?
    };
    rr_update(owner, rtype, rdata)
}

pub fn rr_update(owner: &str, rtype: &str, rdata: &str) -> Response<Body> {
    let rdata = match rdata.parse::<IpAddr>() {
        Ok(rdata) => rdata,
        Err(_) => return bad_request(),
    };

    match (rtype, rdata) {
        ("A", IpAddr::V4(_)) | ("AAAA", IpAddr::V6(_)) => {},
        _ => return bad_request(),
    }

    // TODO Call the DNS client to perform the update!

    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{} {} {}", owner, rtype, rdata).into())
        .unwrap()
}

pub fn bad_request() -> Response<Body> {
    canonical(StatusCode::BAD_REQUEST)
}

pub fn unauthorized() -> Response<Body> {
    let code = StatusCode::UNAUTHORIZED;
    Response::builder()
        .status(code)
        .header(header::WWW_AUTHENTICATE, "Basic realm=\"dynonym\"")
        .body(format!("{}\n", code).into())
        .unwrap()
}

pub fn forbidden() -> Response<Body> {
    canonical(StatusCode::FORBIDDEN)
}

pub fn not_found() -> Response<Body> {
    canonical(StatusCode::NOT_FOUND)
}

pub fn method_not_allowed(methods: &[Method]) -> Response<Body> {
    let code = StatusCode::METHOD_NOT_ALLOWED;
    let methods = methods.iter().map(Method::as_str).collect::<Vec<_>>().join(", ");
    Response::builder()
        .status(code)
        .header(header::ALLOW, methods.as_str())
        .body(format!("{}\n", code).into())
        .unwrap()
}

pub fn internal_server_error() -> Response<Body> {
    canonical(StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn not_implemented() -> Response<Body> {
    canonical(StatusCode::NOT_IMPLEMENTED)
}

fn canonical(code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(code)
        .body(format!("{}\n", code).into())
        .unwrap()
}
