#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    // this is where you pass in your custom commands.
    .invoke_handler(tauri::generate_handler![hash_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use sha2::{Sha256};
use std::{fs, io};
use hex::encode_upper;
use digest::Digest;

#[tauri::command(async)]
async fn hash_command(path: String, algo: String) -> String {
  match &algo as &str {
    "SHA256" => return hash_file(path, Sha256::default()),
    _ => return hash_file(path, Sha256::default()),
  }
}

fn hash_file<T>(path: String, mut hasher: T) -> String
where
  T: Digest,
  T: io::Write, 
{
    let mut input = fs::File::open(path).unwrap();
    io::copy(&mut input, &mut hasher).unwrap();
    return encode_upper(hasher.finalize());
}