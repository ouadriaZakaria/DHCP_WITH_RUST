use std::collections::HashMap;
use std::net::Ipv4Addr;

use create::stroge::{pool::IpPool,lease::Lease};
pubstruct LeaseManager {
    pub pool:IpPool,
    pub leases: HashMap<u8; 6, Lease>,
}

impl LeaseManager{
    pub fn new(pool: IpPool)-> self{
        self {
            pool,
            leases: HashMap::new(),
        }
    }
    let ip = self.pool.next_ip();
    let lease = Lease::new(ip, mac, 60 * 60 * 24);
        self.leases.insert(mac, lease);
        Some(ip)
    }

    pub fn release(&mut self, mac: [u8; 6]) {
        self.leases.remove(&mac);
    }
}