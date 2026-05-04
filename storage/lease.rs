use std::net::Ipv4Addr;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
pub struct Lease {
    pub ip: Ipv4Addr,
    pub mac: [u8; 6],
    pub expires: SystemTime,
}

impl Lease {
    pub fn new(ip: Ipv4Addr, mac: [u8; 6], ttl_secs: u64) -> Self {
        Self {
            ip,
            mac,
            expires: SystemTime::now() + Duration::from_secs(ttl_secs),
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires
    }
}
/*

*/