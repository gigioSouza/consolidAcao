#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub(crate) mod database;
pub(crate) mod config;
mod brokers;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        database::create_database,
        brokers::get_broker_list,
        brokers::new_broker,
        brokers::update_broker
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
