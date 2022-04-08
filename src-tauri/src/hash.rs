// argon2?
use blake2::{Blake2b512, Blake2s256};
use digest;
use hex::encode_upper;
use md5::Md5;
use ripemd::{Ripemd160, Ripemd320};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
// use blake3;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use sm3::Sm3;
use streebog::*;
use tiger::Tiger;
use whirlpool::Whirlpool;

use crate::event;
use crate::file;


#[derive(Clone, serde::Serialize)]
pub struct Hashes {
    path: String,
    value: Vec<Hash>,
}

#[derive(Clone, serde::Serialize)]
pub struct Hash {
    algo: String,
    value: String,
}

// #[tauri::command(async)]
// pub fn hash_files(path: String, algos: Vec<Algorithm>) -> Hashes {
//     let file_hash: Hashes;
//     // for path in &paths {
//     event::emit_event("hash", &path);
//     let mut hashes: Vec<Hash> = Vec::new();
//     for algo in &algos {
//         let hash = hash_file(&path, algo);
//         hashes.push(Hash {
//             algo: algo.to_string(),
//             hash: hash.to_string(),
//         });
//     }
//     file_hash = Hashes {
//         path: path.to_string(),
//         hashes,
//     };
//     // }
//     return file_hash;
// }

// pub fn hash_file(path: &String, algo: &str) -> Hex {
//     match algo {
//         "MD5" => return hasher(path, Md5::default()),
//         "SHA1" => return hasher(path, Sha1::default()),
//         "SHA256" => return hasher(path, Sha256::default()),
//         "SHA384" => return hasher(path, Sha384::default()),
//         "SHA512" => return hasher(path, Sha512::default()),
//         "SHA3-224" => return hasher(path, Sha3_224::new()),
//         "SHA3-256" => return hasher(path, Sha3_256::new()),
//         "SHA3-384" => return hasher(path, Sha3_384::new()),
//         "SHA3-512" => return hasher(path, Sha3_512::new()),
//         "RIPEMD160" => return hasher(path, Ripemd160::default()),
//         "RIPEMD320" => return hasher(path, Ripemd320::default()),
//         "BLAKE2S" => return hasher(path, Blake2s256::default()),
//         "BLAKE2B" => return hasher(path, Blake2b512::default()),
//         "WHIRLPOOL" => return hasher(path, Whirlpool::default()),
//         "SHABAL192" => return hasher(path, Shabal192::new()),
//         "SHABAL224" => return hasher(path, Shabal224::new()),
//         "SHABAL256" => return hasher(path, Shabal256::new()),
//         "SHABAL384" => return hasher(path, Shabal384::new()),
//         "SHABAL512" => return hasher(path, Shabal512::new()),
//         "STREEBOG256" => return hasher(path, Streebog256::new()),
//         "STREEBOG512" => return hasher(path, Streebog512::new()),
//         "TIGER" => return hasher(path, Tiger::default()),
//         "SM3" => return hasher(path, Sm3::default()),
//         "GROESTL" => return hasher(path, Groestl256::default()),
//         "GOST" => return hasher(path, Gost94CryptoPro::default()),
//         "FSB-160" => return hasher(path, Fsb160::default()),
//         "FSB-224" => return hasher(path, Fsb224::default()),
//         "FSB-256" => return hasher(path, Fsb256::default()),
//         "FSB-384" => return hasher(path, Fsb384::default()),
//         "FSB-512" => return hasher(path, Fsb512::default()),
//         _ => return hasher(path, Sha256::default()),
//     }
// }

#[tauri::command(async)]
pub async fn hash_file2(path: String, algo: String) -> Hash {
  println!("hashing file..");
  println!("path: {}", path);
  println!("algo: {}", algo);
  
  let mut read = file::Cacher::new();
  let hasher = Hasher::new(match_algo(&algo));
  let hash = hasher.compute(read.value(&path));
  Hash {
    algo: algo,
    value: hash
  }
  // hash
}

// TODO: 1-1
#[tauri::command(async)]
pub async fn hash_file(path: String, algos: Vec<String>) -> Hashes {
  let mut read = file::Cacher::new();
  let mut hashes: Vec<Hash>= Vec::new();

  for algo in &algos {
    let hasher = Hasher::new(match_algo(algo));
    let hash = hasher.compute(read.value(&path));
    hashes.push(Hash { 
      algo: String::from(algo), 
      value: hash
    });
  }

  Hashes {
    path,
    value: hashes
  }
}

pub struct Hasher<T>
where
T: digest::DynDigest,
T: ?Sized
{
  digest: Box<T>,
}

impl<T> Hasher<T>
where
T: digest::DynDigest,
T: ?Sized
{
  pub fn new (digest: Box<T>) -> Hasher<T> {
    Hasher { digest }
  } 

  pub fn compute(mut self, bytes: &Vec<u8>) -> String {
    self.digest.update(bytes);
    return encode_upper(self.digest.finalize());
  }
}

// understand what this means?
pub fn match_algo(algo: &str) -> Box<dyn digest::DynDigest> {
    match algo {
        "MD5" => return Box::new(Md5::default()),
        "SHA1" => return Box::new(Sha1::default()),
        "SHA256" => return Box::new(Sha256::default()),
        "SHA384" => return Box::new(Sha384::default()),
        "SHA512" => return Box::new(Sha512::default()),
        "SHA3-224" => return Box::new(Sha3_224::new()),
        "SHA3-256" => return Box::new(Sha3_256::new()),
        "SHA3-384" => return Box::new(Sha3_384::new()),
        "SHA3-512" => return Box::new(Sha3_512::new()),
        "RIPEMD160" => return Box::new(Ripemd160::default()),
        "RIPEMD320" => return Box::new(Ripemd320::default()),
        "BLAKE2S" => return Box::new(Blake2s256::default()),
        "BLAKE2B" => return Box::new(Blake2b512::default()),
        "WHIRLPOOL" => return Box::new(Whirlpool::default()),
        "SHABAL192" => return Box::new(Shabal192::new()),
        "SHABAL224" => return Box::new(Shabal224::new()),
        "SHABAL256" => return Box::new(Shabal256::new()),
        "SHABAL384" => return Box::new(Shabal384::new()),
        "SHABAL512" => return Box::new(Shabal512::new()),
        "STREEBOG256" => return Box::new(Streebog256::new()),
        "STREEBOG512" => return Box::new(Streebog512::new()),
        "TIGER" => return Box::new(Tiger::default()),
        "SM3" => return Box::new(Sm3::default()),
        "GROESTL" => return Box::new(Groestl256::default()),
        "GOST" => return Box::new(Gost94CryptoPro::default()),
        "FSB-160" => return Box::new(Fsb160::default()),
        "FSB-224" => return Box::new(Fsb224::default()),
        "FSB-256" => return Box::new(Fsb256::default()),
        "FSB-384" => return Box::new(Fsb384::default()),
        "FSB-512" => return Box::new(Fsb512::default()),
        _ => return Box::new(Md5::default()),
    }
}

// use std::hash;
// use std::io;
// use std::time::Instant;

// fn hasher<T>(path: &String, mut hasher: T) -> Hex
// where
//     T: Digest,
//     T: io::Write,
// {
//     let mut read = file::Cacher::new();
//     // let now = Instant::now();
//     let bytes = read.value(&path);
//     hasher.update(bytes);
//     // let elapsed_time = now.elapsed();
//     // println!("path: {} took {} ms.", path, elapsed_time.as_millis());
//     return encode_upper(hasher.finalize());
// }

// old hash caching code.

// pub fn get_hash(path: &Path, algo: &Algorithm) -> Hex {
//   let mut cache = HASH_CACHE.lock().unwrap();
//   let hashes = cache.get_mut(path);

//   match hashes {
//     Some(hashes) => {
//       // existing path w/ some hashes.
//       let hash = hashes.get(algo);

//       match hash {
//         // we have the file cached.
//         Some(hash) => {
//           emit_event("hash", &[algo, "Found in cache!"].join("-"));
//           hash.to_string()
//         }

//         // we don't have the file cached.
//         None => {
//           emit_event("hash", &[algo, "Not found in cache."].join("-"));
//           let hash = hash_file(path, algo);
//           hashes.insert(algo.to_string(), hash.clone());
//           drop(cache);
//           hash
//         }
//       }
//     }
//     // new file.
//     None => {
//       emit_event("hash", &["No cache found."].join(" "));
//       let hash = hash_file(path, algo);
//       let hash_map: HashMap<Algorithm, Hex> = HashMap::from([(algo.to_string(), hash.clone())]);
//       cache.insert(path.to_string(), hash_map);
//       drop(cache);
//       hash
//     }
//   }
// }
