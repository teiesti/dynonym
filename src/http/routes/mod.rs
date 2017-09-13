pub mod dns;

use std::net::{IpAddr, SocketAddr};

#[get("/ip")]
pub fn ip(addr: SocketAddr) -> String {
    match addr.ip() {
        IpAddr::V4(addr_v4) => format!("{}", addr_v4),
        IpAddr::V6(addr_v6) => format!("{}", addr_v6),
    }
}

#[get("/port")]
pub fn port(addr: SocketAddr) -> String {
    format!("{}", addr.port())
}

#[get("/socket")]
pub fn socket(addr: SocketAddr) -> String {
    match addr {
        SocketAddr::V4(addr_v4) => format!("{}", addr_v4),
        SocketAddr::V6(addr_v6) => format!("{}", addr_v6),
    }
}
