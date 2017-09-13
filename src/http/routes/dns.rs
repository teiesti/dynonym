use model::Credentials;

use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, FromForm)]
struct Update {
    record: String,
    a: Option<Ipv4Addr>,
    aaaa: Option<Ipv6Addr>,
}

#[get("/dns/update?<update>")]
fn update(creds: Credentials, update: Update) -> String {
    format!("{:?}\n{:?}", creds, update)
}
