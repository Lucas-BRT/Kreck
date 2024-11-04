// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod controller;
mod server;
mod ui;
mod utils;
mod window;

fn main() {
    ui::render_tauri_app();
}
