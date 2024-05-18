use std::{sync::Arc, u16};
use tauri::{
    async_runtime::{self, Mutex},
    AppHandle, Manager,
};

use super::rocket_endpoints::*;
use crate::kenku_remote_controller::setup_kenku_controller;

#[tauri::command]
pub async fn request_server_shutdown(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}

#[tauri::command]
pub async fn launch_server(app_state: AppHandle, ip: String, port: u16) {
    let rocket_state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();

    let controller = setup_kenku_controller(ip, port).unwrap();

    let mut rocket_handle = rocket_state.try_lock().unwrap();
    let server = setup_server(controller).await.unwrap();
    let shutdown_handle = server.shutdown();

    rocket_handle.0 = Some(shutdown_handle);

    async_runtime::spawn(async move {
        server.launch().await.unwrap();
    });
}
