use std::net::Ipv4Addr;

pub struct IpPool {
    start: u32,
    end: u32,
    current: u32,
}

impl IpPool {
    pub fn new(start: Ipv4Addr, end: Ipv4Addr) -> self {
        self {
                start: u32::from(start),
                end: u32::from(end),
                current: u32::from(start),
        }
    }
        pub fn next_ip(&mut self) -> Option<Ipv4Addr> {
        if self.current > self.end {
            return None;
        }

        let ip = Ipv4Addr::from(self.current);
        self.current += 1;
        Some(ip)
    }
}