use crate::utils::aplication_state::RocketShutdownHandle;
use crate::server::setup_server;
use crate::controller::setup_kenku_controller;
use crate::utils::local_ip::get_local_ip;
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
            Err(e) => println!("Failed to launch Rocket!: {e}")
        }
    });

    Ok(())
}
