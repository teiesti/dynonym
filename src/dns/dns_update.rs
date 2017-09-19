use errors::*;

use std::net::IpAddr::{self, V4, V6};
use trust_dns::client::{Client, SyncClient};
use trust_dns::op::ResponseCode;
use trust_dns::rr::Name as Domain;
use trust_dns::rr::{DNSClass, Name, RData, Record, RecordType};

pub struct Provider {
    client: SyncClient,
}

impl Provider {
    // TODO
}

impl super::Provider for Provider {
    fn update(&self, domain: Domain, ip: IpAddr) -> Result<()> {
        // Find the zone name to update, i.e. SOA name
        let origin = soa(&self.client, &domain);

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
        let result = self.client.append(record, origin, false).unwrap();    // TODO remove unwrap
        assert_eq!(result.response_code(), ResponseCode::NoError);

        Ok(())
    }
}

#[inline]
fn soa(client: &SyncClient, name: &Name) -> Name {
    for name in
        (0..name.num_labels() + 1)
            .rev()
            .map(|x| name.trim_to(x as usize))
    {
        // TODO remove unwrap
        let response = client.query(&name, DNSClass::IN, RecordType::SOA).unwrap();
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
            assert!(soa.zone_of(&name));
            return soa;
        }
    }
    unreachable!()
}
