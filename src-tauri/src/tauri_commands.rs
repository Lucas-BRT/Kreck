use std::sync::Arc;
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Manager,
};

use super::rocket_endpoints::*;

#[tauri::command]
pub async fn request_server_shutdown(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}

#[tauri::command]
pub async fn launch_server(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let mut handle = state.try_lock().unwrap();
    let server = setup_server().await.unwrap();
    let shutdown_handle = server.shutdown();

    handle.0 = Some(shutdown_handle);

    async_runtime::spawn(async move {
        server.launch().await.unwrap();
    });
}
