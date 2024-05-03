// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{menu::{MenuBuilder, MenuItemBuilder}, tray::TrayIconBuilder, Manager, WindowEvent};
use tauri_plugin_positioner::{WindowExt, Position};

fn main() {

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app|{
            
            let launch_kreck = MenuItemBuilder::new("Launch Kreck")
                    .build(app)?;
            let exit_kreck = MenuItemBuilder::new("Exit Kreck")
                    .build(app)?;

            let menu_tray = MenuBuilder::new(app)
                    .item(&launch_kreck)
                    .item(&exit_kreck)
                    .build()?;

            TrayIconBuilder::new()
                .menu(&menu_tray)
                .on_tray_icon_event(move |app, event| {
                    #[cfg(not(target_os = "linux"))]
                    {
                        let mut win = app.get_webview_window("main").unwrap();
                        let _ = win.as_ref().window().move_window(Position::TrayCenter);
                    }
               })
                .on_menu_event(move |app, event| {
                    if event.id() == launch_kreck.id() {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let win = webview_window.app_handle().get_webview_window("main").unwrap();
                            
                            #[cfg(not(target_os = "linux"))]
                            let _ = win.as_ref().window().move_window(Position::TrayCenter);

                            #[cfg(target_os = "linux")]
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
        .on_window_event(|window, event|  {
            match event {
                window_event => {
                    match window_event {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        },
                        WindowEvent::Focused(false) => {
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        },
                        _ => ()
                    }
                }
            };
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
