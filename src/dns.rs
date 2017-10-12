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

        // Create a new updater
        let updater = Self {
            client: SyncClient::new(conn),
            ttl,
        };

        // Return
        Ok(updater)
    }

    pub fn update(&self, domain: Domain, ip: IpAddr) -> Result<()> {
        // Convert domain into the Trust DNS format
        let domain0 = domain.clone().try_into()
            .chain_err(|| ErrorKind::DnsDomainConvert(domain))?;

        // Find the zone name to update, i.e. SOA name
        let zone = self.find_zone(&domain0)?;

        // Remove the existing record
        {
            // Assemble the (pseudo) record
            let mut record = Record::new();
            record.set_name(domain0.clone());
            record.set_ttl(0);
            match ip {
                V4(_) => record.set_rr_type(RecordType::A),
                V6(_) => record.set_rr_type(RecordType::AAAA),
            };

            // Send the request
            let result = self.client.delete_rrset(record, zone.clone())
                .chain_err(|| ErrorKind::DnsRecordDelete)?;
            if result.response_code() != ResponseCode::NoError {
                bail!(ErrorKind::DnsRecordDelete);  // TODO Add more information about the error!
            }
        }

        // Create a new record
        {
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

            // Send the request
            let result = self.client.create(record, zone)
                .chain_err(|| ErrorKind::DnsRecordCreate)?;
            if result.response_code() != ResponseCode::NoError {
                bail!(ErrorKind::DnsRecordCreate);  // TODO Add more information about the error!
            }
        }

        Ok(())
    }

    fn find_zone(&self, domain: &Name) -> Result<Name> {
        for domain in
            (0..domain.num_labels() + 1)
                .rev()
                .map(|x| domain.trim_to(x as usize))
        {
            let response = self.client.query(&domain, DNSClass::IN, RecordType::SOA)
                .chain_err(|| ErrorKind::DnsRecordQuery)?;
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
