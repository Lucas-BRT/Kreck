use crate::commands::*;
use crate::server::RocketShutdownHandle;

use std::sync::Arc;
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::{
    async_runtime::Mutex,
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};

#[cfg(not(target_os = "linux"))]
use tauri::tray::ClickType;

pub fn render_tauri_app() {

    tauri::Builder::default()
            .plugin(tauri_plugin_positioner::init())
            .setup(|app| {
                let shutdown_handle = Arc::new(Mutex::new(RocketShutdownHandle(None)));

                app.manage(shutdown_handle);

                let launch_kreck = MenuItemBuilder::new("Launch Kreck").build(app).expect("failed to create 'Launch Kreck' menu item");
                let exit_kreck = MenuItemBuilder::new("Exit Kreck").build(app).expect("failed to create 'Exit Kreck' menu item");
                let tray_icon = Image::from_bytes(&include_bytes!("../../icons/icon.ico").to_vec()).expect("failed to create icon from image ../icons/icon.ico");

                #[cfg(target_os = "linux")]
                let menu_tray = MenuBuilder::new(app)
                    .item(&launch_kreck)
                    .item(&exit_kreck)
                    .build()
                    .expect("failed to create menu tray");

                #[cfg(not(target_os = "linux"))]
                let menu_tray = MenuBuilder::new(app)
                    .item(&exit_kreck)
                    .build()
                    .expect("failed to create menu tray");

                TrayIconBuilder::new()
                    .icon(tray_icon)
                    .menu(&menu_tray)
                    .on_tray_icon_event(move |app, event| {
                        #[cfg(not(target_os = "linux"))]
                        {
                            match event.click_type {
                                ClickType::Left => {
                                    if let Some(webview_window) =
                                        app.app_handle().get_webview_window("main")
                                    {
                                        tauri_plugin_positioner::on_tray_event(
                                            app.app_handle(),
                                            &event,
                                        );
                                        let _ = webview_window.show();
                                        let _ = webview_window
                                            .as_ref()
                                            .window()
                                            .move_window(Position::TrayCenter);
                                    }
                                }
                                _ => (),
                            }
                        }
                    })
                    .on_menu_event(move |app, event| {
                        if event.id() == launch_kreck.id() {
                            if let Some(webview_window) = app.get_webview_window("main") {
                                let win = webview_window
                                    .app_handle()
                                    .get_webview_window("main")
                                    .unwrap();
                                let _ = win.as_ref().window().move_window(Position::Center);
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            };
                        } else if event.id() == exit_kreck.id() {
                            if let Some(webview_window) = app.get_webview_window("main") {
                                let _ = webview_window.close();
                                let _ = webview_window.app_handle().exit(0);
                            }
                        }
                    })
                    .build(app)?;

                Ok(())
            })
            .on_window_event(|window, event| {
                match event {
                    window_event => match window_event {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        }
                        WindowEvent::Focused(false) => {
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        }
                        _ => (),
                    },
                };
            })
            .invoke_handler(tauri::generate_handler![
                launch_server,
                request_server_shutdown,
                get_host_local_address
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
