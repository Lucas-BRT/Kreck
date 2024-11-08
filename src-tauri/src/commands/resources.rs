use crate::config::Config;
use crate::config::KenkuRemoteAddress;
use crate::config::CONFIG_FILE_NAME;
use crate::error::Error;
use crate::network::get_local_ip;
use crate::network::NetworkAddress;
use crate::server::DEFAULT_SERVER_LAUNCHING_PORT;
use qrcode::Color;
use qrcode::QrCode;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::Ipv4Addr;
use tauri::{async_runtime::Mutex, AppHandle, Manager};

#[tauri::command]
pub fn get_all_ip_addresses() -> Result<Vec<NetworkAddress>, Error> {
    let ifas = match local_ip_address::list_afinet_netifas() {
        Ok(ifas) => ifas,
        Err(_) => return Err(Error::LocalIpAddressNotFound),
    };

    let addresses: Vec<NetworkAddress> = ifas
        .iter()
        .filter(|element| element.1.is_ipv4())
        .map(|element| NetworkAddress::new(element.0.clone(), element.1.to_canonical()))
        .collect();

    Ok(addresses)
}

#[tauri::command]
pub async fn get_qr_code_as_matrix() -> Vec<Vec<bool>> {
    let ip = get_local_ip();
    let code = QrCode::new(format!("http://{ip}:{DEFAULT_SERVER_LAUNCHING_PORT}")).unwrap();

    let width = code.width();

    let array: Vec<bool> = code
        .to_colors()
        .to_vec()
        .iter()
        .map(|element| match *element {
            Color::Light => false,
            Color::Dark => true,
        })
        .collect();

    let size = array.len() / width;

    let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(size);

    for i in 0..size {
        let begin = i * size;
        let end = begin + size;

        matrix.push(array[begin..end].to_vec());
    }

    matrix
}

#[tauri::command]
pub async fn get_config(handler: AppHandle) -> Config {
    let config = handler.state::<Mutex<Config>>();

    let config = config.lock().await;

    *config
}

#[tauri::command]
pub async fn set_config(handler: AppHandle, address: Ipv4Addr, port: u16) {
    let state = handler.state::<Mutex<Config>>();

    let mut state_config = state.lock().await;

    state_config.set_kenku_remote_address(KenkuRemoteAddress::new(address, port));

    let mut data_path = handler.path().app_data_dir().unwrap();

    data_path.push(CONFIG_FILE_NAME);

    let mut config_file = OpenOptions::new().write(true).open(&data_path).unwrap();

    let json_config = serde_json::to_string_pretty(&*state_config).unwrap();

    config_file.write_all(json_config.as_bytes()).unwrap();
}
