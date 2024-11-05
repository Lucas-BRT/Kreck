use kenku_control::Controller;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream},
    str::FromStr,
    time::Duration,
};

pub async fn setup_kenku_controller(ip: String, port: u16) -> Result<Controller, String> {
    let address = SocketAddrV4::new(
        Ipv4Addr::from_str(ip.as_str()).expect("invalid Ip address"),
        port,
    );

    // todo!: update the kenku_control crate to allow controlling the delay time for server check
    match TcpStream::connect_timeout(&SocketAddr::from(address), Duration::from_millis(50)) {
        Ok(_) => Ok(Controller::new(ip, port)),
        Err(_) => Err("Kenku Remote Offline".to_string()),
    }
}
