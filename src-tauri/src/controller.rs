use kenku_control::Controller;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    str::FromStr,
};

pub async fn setup_kenku_controller(ip: String, port: u16) -> Result<Controller, String> {
    let address = SocketAddrV4::new(
        Ipv4Addr::from_str(ip.as_str()).expect("invalid Ip address"),
        port,
    );

    match kenku_control::utils::check_kenku_server_state(address).await {
        kenku_control::KenkuState::Online => Ok(Controller::new(ip, port)),
        kenku_control::KenkuState::Offline => Err("Server Offline".to_string()),
    }
}
