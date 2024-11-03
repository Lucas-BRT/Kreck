use serde::{Deserialize, Serialize};
use std::{fmt::Display, net::Ipv4Addr};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct KenkuRemoteAddress {
    address: Ipv4Addr,
    port: u16,
}

impl Default for KenkuRemoteAddress {
    fn default() -> Self {
        KenkuRemoteAddress {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 3333,
        }
    }
}

impl Display for KenkuRemoteAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.address, self.port)
    }
}

impl KenkuRemoteAddress {
    pub fn new(address: Ipv4Addr, port: u16) -> Self {
        KenkuRemoteAddress { address, port }
    }

    pub fn get_ip(&self) -> Ipv4Addr {
        self.address
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct Config {
    kenku_remote_address: KenkuRemoteAddress,
}

impl Config {
    pub fn set_kenku_remote_address(&mut self, kenku_remote: KenkuRemoteAddress) {
        self.kenku_remote_address = kenku_remote
    }

    pub fn get_kenku_remote_address(&self) -> KenkuRemoteAddress {
        self.kenku_remote_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_kenku_remote_address() {
        let default_address = KenkuRemoteAddress::default();

        assert_eq!(default_address.to_string(), "127.0.0.1:3333".to_string())
    }

    #[test]
    fn test_default_kenku_remote_address_should_fail() {
        let default_address = KenkuRemoteAddress::default();

        assert_ne!(default_address.to_string(), "0.0.0.0:1".to_string())
    }
}
