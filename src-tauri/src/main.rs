#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use state::Storage;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct FileHash {
  path: String,
  hashes: Vec<Hash>,
}

#[derive(Clone, serde::Serialize)]
struct Hash {
  algo: String,
  hash: String,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

static FILE_MAP: Storage<Mutex<HashMap<String, Vec<u8>>>> = Storage::new();
static MAIN_WINDOW: Storage<Window> = Storage::new();

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Store file map.
      let initial_map = HashMap::new();
      FILE_MAP.set(Mutex::new(initial_map));

      // Store main window.
      let main_window = app.get_window("main").unwrap();
      MAIN_WINDOW.set(main_window);

      Ok(())
    })
    // this is where you pass in your custom commands.
    .invoke_handler(tauri::generate_handler![hash_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn emit_event(event_name: &str, message: &str) {
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

// argon2?
use blake2::{Blake2b512, Blake2s256};
use digest::Digest;
use hex::encode_upper;
use md5::Md5;
use ripemd::{Ripemd160, Ripemd320};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::{fs, io};
// use blake3;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use sm3::Sm3;
use streebog::*;
use tiger::Tiger;
use whirlpool::Whirlpool;

#[tauri::command(async)]
async fn hash_command(paths: Vec<String>, algos: Vec<String>) -> Vec<FileHash> {
  emit_event("test", "hash command init");
  let mut files: Vec<FileHash> = Vec::new();

  for path in &paths {
    let mut hashes: Vec<Hash> = Vec::new();

    for algo in &algos {
      hashes.push(Hash{algo: String::from(algo), hash: get_hash(path, algo)})
    }

    files.push(FileHash{path: String::from(path), hashes})
  }

  return files;
}

fn get_hash(path: &str, algo: &str) -> String
{
  match &algo as &str {
      "MD5" => return hash_file(path, Md5::default()),
      "SHA1" => return hash_file(path, Sha1::default()),
      "SHA256" => return hash_file(path, Sha256::default()),
      "SHA384" => return hash_file(path, Sha384::default()),
      "SHA512" => return hash_file(path, Sha512::default()),
      "SHA3-224" => return hash_file(path, Sha3_224::new()),
      "SHA3-256" => return hash_file(path, Sha3_256::new()),
      "SHA3-384" => return hash_file(path, Sha3_384::new()),
      "SHA3-512" => return hash_file(path, Sha3_512::new()),
      "RIPEMD160" => return hash_file(path, Ripemd160::default()),
      "RIPEMD320" => return hash_file(path, Ripemd320::default()),
      "BLAKE2S" => return hash_file(path, Blake2s256::default()),
      "BLAKE2B" => return hash_file(path, Blake2b512::default()),
      "WHIRLPOOL" => return hash_file(path, Whirlpool::default()),
      "SHABAL192" => return hash_file(path, Shabal192::new()),
      "SHABAL224" => return hash_file(path, Shabal224::new()),
      "SHABAL256" => return hash_file(path, Shabal256::new()),
      "SHABAL384" => return hash_file(path, Shabal384::new()),
      "SHABAL512" => return hash_file(path, Shabal512::new()),
      "STREEBOG256" => return hash_file(path, Streebog256::new()),
      "STREEBOG512" => return hash_file(path, Streebog512::new()),
      "TIGER" => return hash_file(path, Tiger::default()),
      "SM3" => return hash_file(path, Sm3::default()),
      "GROESTL" => return hash_file(path, Groestl256::default()),
      "GOST" => return hash_file(path, Gost94CryptoPro::default()),
      "FSB-160" => return hash_file(path, Fsb160::default()),
      "FSB-224" => return hash_file(path, Fsb224::default()),
      "FSB-256" => return hash_file(path, Fsb256::default()),
      "FSB-384" => return hash_file(path, Fsb384::default()),
      "FSB-512" => return hash_file(path, Fsb512::default()),
      _ => return hash_file(path, Sha256::default()),
    }
}

fn hash_file<T>(path: &str, mut hasher: T) -> String
where
  T: Digest,
  T: io::Write,
{
  // TODO: cache hash
  let bytes = get_bytes(path);
  hasher.update(bytes);
  return encode_upper(hasher.finalize());
}

fn set_bytes(path: &str, bytes: &Vec<u8>) {
  let mut map = FILE_MAP.get().try_lock().unwrap();
  map.insert(path.into(), bytes.to_vec().into());
}

fn get_bytes(path: &str) -> Vec<u8> {
  let map = FILE_MAP.get().lock().unwrap();
  let value = map.get(path);
  // TODO: modify hashmap to use metadata w/ path
  // will not re-read if file has been modified after it has been read once (same path).
  // let md = std::fs::metadata(&file).unwrap();
  match value {
    Some(value) => {
      emit_event("test", "Retreived file cache.");
      let bytes = value.to_vec();
      return bytes;
    }
    None => {
      // If we're here we don't need map.
      drop(map);
      let bytes = read_file(&path);
      emit_event("test", "done reading");
      // needed to get around lock above.
      set_bytes(&path, &bytes);
      return bytes;
    }
  }
}

fn read_file(path: &str) -> Vec<u8> {
  emit_event("test", "Reading file..");
  let mut reader = fs::File::open(path).unwrap();
  let mut writer: Vec<u8> = vec![];
  io::copy(&mut reader, &mut writer).unwrap();
  return writer;
}
