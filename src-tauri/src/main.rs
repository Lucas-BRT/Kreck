// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, menu::{IsMenuItem, MenuBuilder, MenuItemBuilder}, tray::TrayIconBuilder, Manager};

fn main() {

    tauri::Builder::default()
        .setup(|app| {

            let tray_icon  = Image::from_bytes(include_bytes!("../icons/icon.ico"))?;

            // Declared to show tray icon in some Linux distributions;
            #[cfg(target_os = "linux")] 
            {
                let item = MenuItemBuilder::new("Launch Kreck").build(app)?;
                let menu_tray = MenuBuilder::new(app).item(&item).build()?;
                
                TrayIconBuilder::new()
                    .icon(tray_icon)
                    .menu(&menu_tray)
                    .build(app)?;

                app.on_menu_event(move |app, event| {
                    if event.id() == item.id() {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let _ = webview_window.show();
                            let _ = webview_window.set_focus();
                        };
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
