use super::tray::setup_tray;
use crate::config::Config;
use crate::server::RocketShutdownHandle;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{fs::create_dir, sync::Arc};
#[cfg(not(target_os = "linux"))]
use tauri::tray::MouseButton;
use tauri::App;
use tauri::{async_runtime::Mutex, Manager};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let shutdown_handle = Arc::new(Mutex::new(RocketShutdownHandle(None)));

    app.manage(shutdown_handle);

    setup_tray(app.app_handle())?;

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
