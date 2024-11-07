use crate::config::Config;
use crate::server::RocketShutdownHandle;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs::create_dir, sync::Arc};
#[cfg(not(target_os = "linux"))]
use tauri::tray::MouseButton;
use tauri::App;
use tauri::{
    async_runtime::Mutex,
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub const PNG_TRAY_ICON: &[u8] = include_bytes!("../../icons/128x128.png");

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let shutdown_handle = Arc::new(Mutex::new(RocketShutdownHandle(None)));

    app.manage(shutdown_handle);

    let launch_kreck = MenuItemBuilder::new("Launch Kreck")
        .build(app)
        .expect("failed to create 'Launch Kreck' menu item");
    let exit_kreck = MenuItemBuilder::new("Exit Kreck")
        .build(app)
        .expect("failed to create 'Exit Kreck' menu item");
    let tray_icon = Image::from_bytes(PNG_TRAY_ICON)
        .expect("failed to create icon from image ../icons/128x128.png");

    let menu_tray = MenuBuilder::new(app);

    #[cfg(target_os = "linux")]
    let menu_tray = menu_tray.item(&launch_kreck);

    let menu_tray = menu_tray
        .item(&exit_kreck)
        .build()
        .expect("failed to create menu tray");

    TrayIconBuilder::new()
        .icon(tray_icon)
        .menu(&menu_tray)
        .on_tray_icon_event(|tray, event| {
            #[cfg(not(target_os = "linux"))]
            {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let app = tray.app_handle();
                    if let Some(webview_window) = app.app_handle().get_webview_window("main") {
                        tauri_plugin_positioner::on_tray_event(app.app_handle(), &event);
                        let _ = webview_window.show();
                        let _ = webview_window
                            .as_ref()
                            .window()
                            .move_window(Position::TrayCenter);
                    }
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
                    webview_window.app_handle().exit(0);
                }
            }
        })
        .build(app)?;

    // create data folder
    let mut data_path = app.path().app_data_dir().unwrap();

    if !data_path.exists() {
        create_dir(&data_path).unwrap();
    }

    // create config file
    data_path.push("config.json");

    if !data_path.exists() {
        let config = Config::default();

        let mut config_file = File::create(&data_path).unwrap();

        let json_config = serde_json::to_string_pretty(&config).unwrap();

        config_file.write_all(json_config.as_bytes()).unwrap();
    }

    // read config file and load in the state

    let file = File::open(data_path).unwrap();

    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader).unwrap();

    app.manage(Mutex::new(config));

    Ok(())
}
