use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkAddress {
    network_name: String,
    address: IpAddr,
}

impl NetworkAddress {
    pub fn new(network_name: String, address: IpAddr) -> Self {
        NetworkAddress {
            network_name,
            address,
        }
    }
}

pub fn get_local_ip() -> IpAddr {
    local_ip_address::local_ip().expect("failed to get local ip")
}
