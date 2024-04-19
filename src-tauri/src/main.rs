// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, menu::{MenuBuilder, MenuItemBuilder}, tray::TrayIconBuilder, Manager, WindowEvent};

fn main() {

    tauri::Builder::default()
        .setup(|app| {

            let tray_icon  = Image::from_bytes(include_bytes!("../icons/icon.ico"))?;

            // Declared to show tray icon in some Linux distributions;
            #[cfg(target_os = "linux")] 
            {
                let launch_kreck = MenuItemBuilder::new("Launch Kreck")
                    .build(app)?;
                let exit_kreck = MenuItemBuilder::new("Exit Kreck")
                    .build(app)?;

                let menu_tray = MenuBuilder::new(app)
                    .item(&launch_kreck)
                    .item(&exit_kreck)
                    .build()?;
                
                TrayIconBuilder::new()
                    .icon(tray_icon)
                    .menu(&menu_tray)
                    .build(app)?;

                app.on_menu_event(move |app, event| {
                    if event.id() == launch_kreck.id() {
                        if let Some(webview_window) = app.get_webview_window("main") {
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
            }

            #[cfg(not(target_os = "linux"))]
            {
                let menu_tray = MenuBuilder::new(app).build()?;

                TrayIconBuilder::new()
                    .icon(tray_icon)
                    .menu(&menu_tray)
                    .on_tray_icon_event(|tray, event| {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        }
                    })
                    .build(app)?;
            }

            Ok(())
        })
        .on_window_event(|window, event|  {
            match event {
                window_event => {
                    match window_event {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        }
                        _ => ()
                    }
                }
            };
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
