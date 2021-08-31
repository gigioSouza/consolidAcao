#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub(crate) mod database;
pub(crate) mod config;
mod brokers;
mod brokerage_note;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        database::create_database,
        brokers::get_broker_list,
        brokers::new_broker,
        brokers::update_broker,
        brokerage_note::get_brokerage_note_page,
        brokerage_note::get_brokerage_note_orders,
        brokerage_note::new_brokerage_note,
        brokerage_note::update_brokerage_note,
        brokerage_note::delete_brokerage_note
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
