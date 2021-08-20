#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod config;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        config::check_config,
        config::create_config_dir,
        config::create_database,
        config::get_config_dir_path
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
