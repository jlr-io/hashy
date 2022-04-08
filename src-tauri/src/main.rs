#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use state::Storage;
use tauri::{Manager, Window};

mod hash;
mod file;
mod event;

static MAIN_WINDOW: Storage<Window> = Storage::new();

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Store main window.
      let main_window = app.get_window("main").unwrap();
      MAIN_WINDOW.set(main_window);

      Ok(())
    })
    // this is where you pass in your custom commands.
    .invoke_handler(tauri::generate_handler![hash::hash_file, hash::hash_file2, file::get_file_metadata])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

