pub mod setup;
pub mod tray;
pub mod window;

use crate::commands::{resources::*, server::*, window::*};
use setup::setup;
use window::events::handle_window_events;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(setup)
        .on_window_event(handle_window_events)
        .invoke_handler(tauri::generate_handler![
            launch_server,
            request_server_shutdown,
            get_all_ip_addresses,
            open_qr_code_window,
            open_config_window,
            get_qr_code_as_matrix,
            get_config,
            set_config,
            close_window,
            open_error_window,
            emit_error,
            open_issues_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
