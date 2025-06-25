use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Digest, Sha256};
use blake3;
use xxhash_rust::xxh3::Xxh3;
use crate::cli::HashAlgorithm;

fn read_file_in_chunks<F>(path: &Path, mut update_fn: F) -> Result<(), std::io::Error>
where
    F: FnMut(&[u8]),
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        update_fn(&buffer[..bytes_read]);
    }
    Ok(())
}

pub fn hash_file(path: &Path, algo: &HashAlgorithm) -> Result<String, std::io::Error> {
    match algo {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            read_file_in_chunks(path, |chunk| { hasher.update(chunk); })?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            read_file_in_chunks(path, |chunk| { hasher.update(chunk); })?;
            Ok(hasher.finalize().to_hex().to_string())
        }
        HashAlgorithm::Xxhash => {
            let mut hasher = Xxh3::new();
            read_file_in_chunks(path, |chunk| { hasher.update(chunk); })?;
            Ok(format!("{:x}", hasher.digest()))
        }
    }
}


