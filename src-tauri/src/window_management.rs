use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub fn create_window(
    handler: &AppHandle,
    label: &str,
    url: WebviewUrl,
    window_width: u32,
    window_height: u32,
) -> Result<WebviewWindow, tauri::Error> {
    let primary_monitor = handler
        .primary_monitor()
        .expect("Failed to get the primary monitor");

    let monitor = primary_monitor.expect("Primary monitor not avaliable");
    let monitor_size = monitor.size();

    let window_x_position = (monitor_size.width / 2) - (window_width / 2);
    let window_y_position = (monitor_size.height / 2) - (window_height / 2);

    // create the window with default parameters
    WebviewWindowBuilder::new(handler, label, url)
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .resizable(false)
        .maximizable(false)
        .inner_size(window_width.into(), window_height.into())
        .position(window_x_position.into(), window_y_position.into())
        .shadow(false)
        .build()
}
