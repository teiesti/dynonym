use super::Domain;
use errors::*;

use std::net::IpAddr::{self, V4, V6};
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::ResponseCode;
use trust_dns::rr::{DNSClass, RData, Record, RecordType};

pub struct Provider {
    client: SyncClient,
}

impl Provider {
    // TODO

    #[inline]
    fn find_zone(&self, domain: &Domain) -> Result<Domain> {
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

impl super::Provider for Provider {
    fn update(&self, domain: Domain, ip: IpAddr) -> Result<()> {
        // Find the zone name to update, i.e. SOA name
        let zone = self.find_zone(&domain)
            .chain_err(|| "Could not find zone for given domain")?;

        // Assemble the record
        let mut record = Record::new();
        record.set_name(domain);
        record.set_ttl(60); // TODO Take value from config!
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
            .chain_err(|| "Could not update given domain")?;
        assert_eq!(result.response_code(), ResponseCode::NoError);

        Ok(())
    }
}
