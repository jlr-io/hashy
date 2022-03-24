use std::collections::HashMap;

pub type Path = String;
pub type Bytes = Vec<u8>;

use std::{fs, io};

pub fn read_file(path: &String) -> Bytes {
    let mut reader = fs::File::open(path).unwrap();
    let mut bytes: Bytes = vec![];
    io::copy(&mut reader, &mut bytes).unwrap();
    bytes
}

pub struct FileCacher<T>
where
    T: Fn(&String) -> Vec<u8>,
{
    reader: T,
    value: HashMap<String, Vec<u8>>,
}

impl<T> FileCacher<T>
where
    T: Fn(&String) -> Vec<u8>,
{
    pub fn new(reader: T) -> FileCacher<T> {
        FileCacher {
            reader,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: &String) -> Vec<u8> {
        let bytes = match self.value.get(arg) {
            Some(v) => v.to_vec(),
            None => {
                let v = (self.reader)(arg);
                self.value.insert(arg.to_string(), v.clone());
                v
            }
        };
        bytes
    }
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
