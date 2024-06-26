// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hermes::{connect::__cmd__connect_command, find_ip_address::{__cmd__find_ip_address, find_ip_address}, my_custom_command::__cmd__my_custom_command};
use hermes::my_custom_command::my_custom_command;
use hermes::connect::connect_command;



use std::io::Cursor;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command, find_ip_address, connect_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
