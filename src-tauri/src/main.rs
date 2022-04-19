#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod hash;
mod file;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hash::hash_file, file::get_file_metadata])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

