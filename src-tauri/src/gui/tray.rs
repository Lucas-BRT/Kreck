use tauri::tray::TrayIcon;
use tauri::AppHandle;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub const PNG_TRAY_ICON: &[u8] = include_bytes!("../../icons/128x128.png");

pub fn setup_tray(handler: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    let launch_kreck = MenuItemBuilder::new("Launch Kreck")
        .build(handler)
        .expect("failed to create 'Launch Kreck' menu item");
    let exit_kreck = MenuItemBuilder::new("Exit Kreck")
        .build(handler)
        .expect("failed to create 'Exit Kreck' menu item");
    let tray_icon = Image::from_bytes(PNG_TRAY_ICON)
        .expect("failed to create icon from image ../icons/128x128.png");

    // build menu tray and only add the launch button if the platform is linux
    let menu_tray = MenuBuilder::new(handler);
    #[cfg(target_os = "linux")]
    let menu_tray = menu_tray.item(&launch_kreck);
    let menu_tray = menu_tray
        .item(&exit_kreck)
        .build()
        .expect("failed to create menu tray");

    // add support to open the tray
    let tray = TrayIconBuilder::new().icon(tray_icon).menu(&menu_tray);
    #[cfg(not(target_os = "linux"))]
    let tray = tray.on_tray_icon_event(|tray, event| {
        use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = event
        {
            let handler = tray.app_handle();
            if let Some(webview_window) = handler.app_handle().get_webview_window("main") {
                tauri_plugin_positioner::on_tray_event(handler, &event);
                webview_window.show().expect("failed to show main window");
                webview_window
                    .move_window(Position::TrayCenter)
                    .expect("failed to move window");
            }
        }
    });

    // add support to open the tray in a linux Gnome enviroment
    let tray = tray.on_menu_event(move |handler, event| {
        if event.id() == launch_kreck.id() {
            if let Some(webview_window) = handler.get_webview_window("main") {
                webview_window
                    .move_window(Position::Center)
                    .expect("failed to move window");
                webview_window.show().expect("failed to show main window");
                webview_window
                    .set_focus()
                    .expect("failed to focus at the main window");
            };
        } else if event.id() == exit_kreck.id() {
            if let Some(webview_window) = handler.get_webview_window("main") {
                webview_window.close().expect("failed to close main window");
                // close the application
                webview_window.app_handle().exit(0);
            }
        }
    });

    tray.build(handler)
}
