use crate::config::Config;
use crate::config::KenkuRemoteAddress;
use crate::controller::setup_kenku_controller;
use crate::server::setup_server;
use crate::utils::get_local_ip;
use crate::utils::RocketShutdownHandle;
use crate::window::window_management::create_window;
use qrcode::Color;
use qrcode::QrCode;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::Ipv4Addr;
use std::sync::Arc;
use tauri::WebviewUrl;
use tauri::Window;
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Manager,
};

#[tauri::command]
pub fn get_host_local_address() -> Result<String, String> {
    get_local_ip()
}

#[tauri::command]
pub async fn request_server_shutdown(handler: AppHandle) {
    let state = handler.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}

#[tauri::command]
pub async fn launch_server(handler: AppHandle, ip: String, port: u16) -> Result<(), String> {
    let rocket_state = handler.state::<Arc<Mutex<RocketShutdownHandle>>>();

    let controller = setup_kenku_controller(ip, port).await?;

    let mut rocket_handle = rocket_state.try_lock().unwrap();
    let server = setup_server(controller).await.unwrap();
    let shutdown_handle = server.shutdown();

    rocket_handle.0 = Some(shutdown_handle);

    async_runtime::spawn(async move {
        match server.launch().await {
            Ok(_) => println!("Rocket launched successfully!"),
            Err(e) => println!("Failed to launch Rocket!: {e}"),
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn open_qr_code_window(handler: AppHandle) {
    let window_width = 210;
    let window_height = 250;

    create_window(
        &handler,
        "qr-code",
        WebviewUrl::App("pages/qrcode.html".into()),
        window_width,
        window_height,
    )
    .expect("Failed to create qr-code Window.");
}

#[tauri::command]
pub async fn open_config_window(handler: AppHandle) {
    let window_width = 250;
    let window_height = 180;

    create_window(
        &handler,
        "config",
        WebviewUrl::App("pages/config.html".into()),
        window_width,
        window_height,
    )
    .expect("Failed to create config Window.");
}

#[tauri::command]
pub async fn open_error_window(handler: AppHandle) {
    let window_width = 210;
    let window_height = 120;

    // bug: on Fedora linux Gnome, if the method .resizable(true) is called the window not respect the width and height
    // need to open a issue in tauri
    create_window(
        &handler,
        "error",
        WebviewUrl::App("/pages/error.html".into()),
        window_width,
        window_height,
    )
    .expect("Failed to create error Window.")
    .set_resizable(false)
    .expect("failed to set the error window to be resizable.");
}

#[tauri::command]
pub async fn get_qr_code_as_matrix() -> Vec<Vec<bool>> {
    let ip = get_local_ip().unwrap();
    let code = QrCode::new(format!("http://{ip}:8000")).unwrap();

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
    // set the current config on state
    let state = handler.state::<Mutex<Config>>();

    let mut state_config = state.lock().await;

    state_config.set_kenku_remote_address(KenkuRemoteAddress::new(address, port));

    // save the current config on config file

    let mut data_path = handler.path().app_data_dir().unwrap();

    data_path.push("config.json");

    let mut config_file = OpenOptions::new().write(true).open(&data_path).unwrap();

    let json_config = serde_json::to_string_pretty(&*state_config).unwrap();

    config_file.write_all(json_config.as_bytes()).unwrap();
}

#[tauri::command]
pub async fn close_window(window: Window) {
    window.destroy().expect("failed to destroy the window");
}

#[tauri::command]
pub fn open_issues_page() {
    let url = "https://github.com/Lucas-BRT/Kreck/issues";

    open::that(url).expect("failed to open browser.");
}
