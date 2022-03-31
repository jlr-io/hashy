use std::collections::HashMap;
use std::{fs, io};

pub fn read_file(path: &String) -> Vec<u8> {
    println!("reading file..");
    let mut reader = fs::File::open(path).unwrap();
    let mut bytes = vec![];
    io::copy(&mut reader, &mut bytes).unwrap();
    bytes
}

pub struct Cacher
{
    map: HashMap<String, Vec<u8>>,
}

impl Cacher
{
    pub fn new() -> Cacher {
        Cacher {
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: &String) -> &Vec<u8> {
        let bytes = self.map
            .entry(String::from(arg))
            .or_insert_with(||read_file(arg));
        bytes
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FileMetaData {
    name: String,
    path: String,
    file_type: String,
    size: String,
    last_modified: String,
}

impl FileMetaData {
    pub fn get(path: String) -> FileMetaData {
        let metadata = fs::metadata(&path).expect("unable to read file");

    let name = String::from(path.split("/").last().unwrap());
    let file_type = if metadata.is_file() { String::from("file") } else { String::from("dir")};
    let last_modified = readable_time(metadata.modified().unwrap());
    let kb = metadata.len() / 1000;
    // let size = kb.to_string().push_str("KB");   
    let size = format!("{}KB", kb);
    FileMetaData {
        name,
        path, 
        file_type, 
        last_modified, 
        size
    }
    }
}

#[tauri::command]
pub fn get_file_metadata(path: String) -> FileMetaData {
    FileMetaData::get(path)
}

use chrono;
use std::time;

pub fn readable_time(time: time::SystemTime) -> String {
    let datetime = chrono::DateTime::<chrono::Utc>::from(time);
    let timestamp_str = datetime.format("%m-%d-%Y %H:%M:%S").to_string();
    timestamp_str
}
// old caching code.

// pub type FileBytes = HashMap<Path, Bytes>;

// lazy_static! {
//   static ref FILE_CACHE: Mutex<FileBytes> = Mutex::new(HashMap::new());
// }

// pub fn get_bytes(path: &String) -> Bytes {
//   let mut map = FILE_CACHE.lock().unwrap();
//   let file = map.get(path);
//   match file {
//     Some(file) => {
//       emit_event("file", &["Found cache!"].join(" "));
//       let bytes = file.to_vec();
//       drop(map);
//       bytes
//     }
//     None => {
//       emit_event("file", &["No cache found."].join(" "));
//       let bytes = read_file(path);
//       map.insert(path.to_string(), bytes.to_vec());
//       drop(map);
//       bytes
//     }
//   }
// }
