use crate::gui::window::window_management::create_window;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Listener;
use tauri::WebviewUrl;
use tauri::Window;

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
pub fn close_window(window: Window) {
    window.destroy().expect("failed to destroy the window");
}

#[tauri::command]
pub fn hide_window(window: Window) {
    window.hide().expect("failed to hide the window");
}

#[tauri::command]
pub fn open_issues_page() {
    let url = "https://github.com/Lucas-BRT/Kreck/issues";

    open::that(url).expect("failed to open browser.");
}

#[tauri::command]
pub async fn emit_error(handler: AppHandle, message: String) {
    let inner_handler = handler.clone();

    handler.listen("error-window-ready", move |_| {
        inner_handler
            .emit("error", &message)
            .expect("failed to emit error event");
    });
}
