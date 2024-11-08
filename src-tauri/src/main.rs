// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod config;
mod controller;
mod error;
mod gui;
mod network;
mod server;

fn main() {
    gui::run();
}
