use tauri::Window;
use tauri::{Manager, WindowEvent};

use super::window_management::hide_window;

pub fn handle_window_events(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            if let Some(webview_window) = window.get_webview_window("main") {
                hide_window(webview_window);
            };
        }
        WindowEvent::Focused(false) => {
            if let Some(webview_window) = window.get_webview_window("main") {
                hide_window(webview_window);
            };
        }
        _ => (),
    };
}
