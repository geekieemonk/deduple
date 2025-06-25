use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Digest, Sha256};
use blake3;
use xxhash_rust::xxh3::Xxh3;
use crate::cli::HashAlgorithm;

pub fn hash_file(path: &Path, algo: &HashAlgorithm) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    match algo {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }
        HashAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }
        HashAlgorithm::Xxhash => {
            let mut hasher = Xxh3::new();
            let mut buffer = [0u8; 8192];
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Ok(format!("{:x}", hasher.digest()))
        }
    }
}

pub fn compare_files(file1: &Path, file2: &Path, algo: &HashAlgorithm) -> Result<bool, std::io::Error> {
    let hash1 = hash_file(file1, algo)?;
    let hash2 = hash_file(file2, algo)?;
    Ok(hash1 == hash2)
}
