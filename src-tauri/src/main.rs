// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use kenku_control::{self, Controller};
use rocket::{fs::FileServer, get, Ignite, Rocket, Shutdown};
use std::sync::Arc;
use tauri::{
    async_runtime::{self, Mutex},
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

#[derive(Default)]
struct RocketShutdownHandle(pub Option<Shutdown>);

#[tauri::command]
async fn request_server_shutdown(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let handle = state.try_lock().unwrap();

    if let Some(handler) = &handle.0 {
        handler.clone().notify();
    }
}

#[tauri::command]
async fn setup_server() -> Result<Rocket<Ignite>, rocket::Error> {
    let rocket_config = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000))
        .merge(("workers", 4));

    rocket::custom(rocket_config)
        .mount("/", FileServer::from("../src"))
        .ignite()
        .await
}

#[tauri::command]
async fn launch_server(app_state: AppHandle) {
    let state = app_state.state::<Arc<Mutex<RocketShutdownHandle>>>();
    let mut handle = state.try_lock().unwrap();
    let server = setup_server().await.unwrap();
    let shutdown_handle = server.shutdown();

    handle.0 = Some(shutdown_handle);

    async_runtime::spawn(async move {
        server.launch().await.unwrap();
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let shutdown_handle = Arc::new(Mutex::new(RocketShutdownHandle(None)));

            app.manage(shutdown_handle);

            let launch_kreck = MenuItemBuilder::new("Launch Kreck").build(app)?;
            let exit_kreck = MenuItemBuilder::new("Exit Kreck").build(app)?;

            let tray_icon = Image::from_bytes(&include_bytes!("../icons/icon.ico").to_vec())
                .expect("failed to create icon from image ../icons/icon.ico");

            #[cfg(target_os = "linux")]
            let menu_tray = MenuBuilder::new(app)
                .item(&launch_kreck)
                .item(&exit_kreck)
                .build()?;

            #[cfg(not(target_os = "linux"))]
            let menu_tray = MenuBuilder::new(app).item(&exit_kreck).build()?;

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
                window_event => {
                    match window_event {
                        WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            if let Some(webview_window) = window.get_webview_window("main") {
                                webview_window.hide().unwrap();
                            };
                        }
                        // WindowEvent::Focused(false) => {
                        //     if let Some(webview_window) = window.get_webview_window("main") {
                        //         webview_window.hide().unwrap();
                        //     };
                        // },
                        _ => (),
                    }
                }
            };
        })
        .invoke_handler(tauri::generate_handler![
            launch_server,
            request_server_shutdown
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
