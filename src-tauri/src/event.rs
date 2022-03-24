use super::MAIN_WINDOW;
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

pub fn emit_event(event_name: &str, message: &str) {
  println!("{} - {}", event_name, message);
  // app instead?
  let main_window = MAIN_WINDOW.get();
  main_window
    .emit_all(
      event_name,
      Payload {
        message: message.into(),
      },
    )
    .unwrap();
}
