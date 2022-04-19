use std::collections::HashMap;
use std::{fs, io};

pub fn read_file(path: &String) -> Result<Vec<u8>, io::Error> {
    let reader = fs::File::open(path);

    let mut reader = match reader {
        Ok(file) => file,
        Err(error) => return Err(error)
    };

    let mut bytes = vec![];

    match io::copy(&mut reader, &mut bytes) {
        Ok(_) => return Ok(bytes),
        Err(error) => return Err(error)
    };
}

pub struct Cacher
{
    map: HashMap<String, Result<Vec<u8>, io::Error>>,
}

impl Cacher
{
    pub fn new() -> Cacher {
        Cacher {
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: &String) -> Result<&[u8], String> {
        let bytes = self
            .map
            .entry(String::from(arg))
            .or_insert_with(|| read_file(arg))
            .as_deref();

        match bytes {
            Ok(bytes) => return Ok(bytes),
            Err(_) => return Err("There was an error reading the file".into())
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FileMetaData {
    name: String,
    path: String,
    size: String,
    last_modified: String,
}

impl FileMetaData {
    pub fn get(path: String) -> FileMetaData {
        let metadata = fs::metadata(&path).expect("unable to read file");
        let name = String::from(path.split("/").last().unwrap());
        let last_modified = readable_time(metadata.modified().unwrap());
        let kb = metadata.len() / 1000;
        let size = format!("{}KB", kb);
        FileMetaData {
            name,
            path,
            last_modified,
            size,
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
