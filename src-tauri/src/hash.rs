use blake2::{Blake2b512, Blake2s256};
use digest;
use fsb::{Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};
use gost94::Gost94CryptoPro;
use groestl::Groestl256;
use hex::encode_upper;
use md5::Md5;
use ripemd::{Ripemd160, Ripemd320};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512};
use sm3::Sm3;
use streebog::*;
use tiger::Tiger;
use whirlpool::Whirlpool;

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

#[tauri::command]
pub fn hash_file(path: String, algo: String) -> Result<Hash, String> {
    let mut reader = file::Cacher::new();
    let hasher = Hasher::new(match_algo(&algo));

    let bytes = reader.value(&path);

    let hash = match bytes {
      Ok(bytes) => hasher.compute(bytes),
      Err(error) => return Err(error)
    };
    
    Ok(Hash {
        algo: algo,
        value: hash,
    })
}

pub struct Hasher<T>
where
    T: digest::DynDigest,
    T: ?Sized,
{
    digest: Box<T>,
}

impl<T> Hasher<T>
where
    T: digest::DynDigest,
    T: ?Sized,
{
    pub fn new(digest: Box<T>) -> Hasher<T> {
        Hasher { digest }
    }

    pub fn compute(mut self, bytes: &[u8]) -> String {
        self.digest.update(bytes);
        return encode_upper(self.digest.finalize());
    }
}

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
