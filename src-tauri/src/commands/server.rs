use crate::controller::setup_kenku_controller;
use crate::server::setup_server;
use crate::server::RocketShutdownHandle;
use std::sync::Arc;
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Manager,
};

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
pub async fn request_server_shutdown(handler: AppHandle) {
    let state = handler.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}
