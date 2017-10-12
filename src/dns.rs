//! Domain name system update client (RFC 2136 "DNS UPDATE")

use errors::*;
use types::Domain;

use std::convert::TryInto;
use std::net::IpAddr::{self, V4, V6};
use std::net::SocketAddr;
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::ResponseCode;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};
use trust_dns::udp::UdpClientConnection;

pub struct Updater {
    client: SyncClient,
    ttl: u32,
}

impl Updater {
    pub fn new(socket: SocketAddr, ttl: u32) -> Result<Self> {
        // Open a connection
        let conn = UdpClientConnection::new(socket)
            .chain_err(|| ErrorKind::DnsConnOpen(socket))?;

        // Create a new provider
        let provider = Self {
            client: SyncClient::new(conn),
            ttl,
        };

        // Return
        Ok(provider)
    }

    pub fn update(&self, domain: Domain, ip: IpAddr) -> Result<()> {
        // Convert domain into the Trust DNS format
        let domain0 = domain.clone().try_into()
            .chain_err(|| ErrorKind::DnsConvertDomain(domain.clone()))?;

        // Find the zone name to update, i.e. SOA name
        let zone = self.find_zone(&domain0)
            .chain_err(|| ErrorKind::DnsFindZone(domain.clone()))?;

        // Assemble the record
        let mut record = Record::new();
        record.set_name(domain0);
        record.set_ttl(self.ttl);
        match ip {
            V4(ipv4) => {
                record.set_rr_type(RecordType::A);
                record.set_rdata(RData::A(ipv4));
            },
            V6(ipv6) => {
                record.set_rr_type(RecordType::AAAA);
                record.set_rdata(RData::AAAA(ipv6));
            }
        }

        // Send the update request
        let result = self.client.append(record, zone, false)
            .chain_err(|| ErrorKind::DnsUpdate(domain, ip))?;
        assert_eq!(result.response_code(), ResponseCode::NoError);  // TODO handle as error

        Ok(())
    }

    fn find_zone(&self, domain: &Name) -> Result<Name> {
        for domain in
        (0..domain.num_labels() + 1)
        .rev()
        .map(|x| domain.trim_to(x as usize))
        {
            let response = self.client.query(&domain, DNSClass::IN, RecordType::SOA)?;
            let record =
                response
                    .answers().iter()
                    .chain(
                        response
                            .name_servers().iter()
                            .filter(|x| x.rr_type() == RecordType::SOA)
                    )
                    .next()
            ;
            if let Some(record) = record {
                let soa = record.name().clone();
                assert!(soa.zone_of(&domain));
                return Ok(soa);
            }
        }
        unreachable!()
    }
}
