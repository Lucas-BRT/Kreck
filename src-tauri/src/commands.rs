use crate::controller::setup_kenku_controller;
use crate::server::setup_server;
use crate::utils::get_local_ip;
use crate::utils::RocketShutdownHandle;
use qrcode::QrCode;
use std::sync::Arc;
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Manager,
};

#[tauri::command]
pub fn get_host_local_address() -> Result<String, String> {
    get_local_ip()
}

#[tauri::command]
pub async fn request_server_shutdown(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}

#[tauri::command]
pub async fn launch_server(app_state: AppHandle, ip: String, port: u16) -> Result<(), String> {
    let rocket_state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();

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
    let monitor = handler.primary_monitor().unwrap().unwrap();
    let position = monitor.size();

    let window_width = 210;
    let window_height = 250;

    let window_x_position = (position.width / 2) - (window_width / 2);
    let window_y_position = (position.height / 2) - (window_height / 2);

    tauri::WebviewWindowBuilder::new(
        &handler,
        "QR-Code",
        tauri::WebviewUrl::App("qrcode.html".into()),
    )
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .inner_size(window_width.into(), window_height.into())
    .position(window_x_position.into(), window_y_position.into())
    .shadow(false)
    .build()
    .unwrap();
}

#[tauri::command]
pub async fn open_config_window(handler: AppHandle) {
    let monitor = handler.primary_monitor().unwrap().unwrap();
    let position = monitor.size();

    let window_width = 250;
    let window_height = 180;

    let window_x_position = (position.width / 2) - (window_width / 2);
    let window_y_position = (position.height / 2) - (window_height / 2);

    tauri::WebviewWindowBuilder::new(
        &handler,
        "Config",
        tauri::WebviewUrl::App("config.html".into()),
    )
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .inner_size(window_width.into(), window_height.into())
    .position(window_x_position.into(), window_y_position.into())
    .shadow(false)
    .build()
    .unwrap();
}

#[tauri::command]
pub async fn get_qr_code_as_matrix(handler: AppHandle) -> Vec<Vec<bool>> {
    let code = QrCode::new(b"http://qr-code-placeholder").unwrap();

    let width = code.width();

    // todo: "Update this decapred method."
    let grid = code.into_vec().clone();

    let size = grid.len() / width;

    let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(size);

    for i in 0..size {
        let begin = i * size;
        let end = begin + size;

        matrix.push(grid[begin..end].to_vec());
    }

    matrix
}
