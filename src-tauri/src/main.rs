#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    // this is where you pass in your custom commands. 
    .invoke_handler(tauri::generate_handler![hash_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// custom commands must have #[tauri::command] annotation.
#[tauri::command]
fn hash_file(file_path: String, algorithm: String) -> String {
  println!("{}", file_path);
  println!("{}", algorithm);

  return String::from("test");
}
