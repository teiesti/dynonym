pub mod dns_update;

use errors::*;

use std::net::IpAddr;
pub use trust_dns::rr::Name as Domain;

trait Provider {
    fn update(&self, domain: Domain, ip: IpAddr) -> Result<()>;
}
