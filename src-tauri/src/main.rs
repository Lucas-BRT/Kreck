// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{image::Image, tray::ClickType, Manager};

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            let tray_icon  = Image::from_bytes(include_bytes!("../icons/icon.ico"))?;
    
            // Declared to show tray icon in some Linux distributions;
            let menu_tray = tauri::menu::MenuBuilder::new(app).build()?;
    
            tauri::tray::TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu_tray)
                .build(app)?;
            Ok(())

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
