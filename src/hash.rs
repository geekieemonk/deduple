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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    

    fn write_temp_file(contents: &[u8]) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(contents).unwrap();
        file
    }

    #[test]
    fn test_read_file_in_chunks_success() {
        let file = write_temp_file(b"hello world");
        let mut collected = Vec::new();
        let res = read_file_in_chunks(file.path(), |chunk| collected.extend_from_slice(chunk));
        assert!(res.is_ok());
        assert_eq!(collected, b"hello world");
    }

    #[test]
    fn test_read_file_in_chunks_file_not_found() {
        let res = read_file_in_chunks(std::path::Path::new("/nonexistent/file.txt"), |_| {});
        assert!(res.is_err());
    }

    #[test]
    fn test_hash_file_sha256() {
        let file = write_temp_file(b"abc");
        let hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "abc" | sha256sum
        assert_eq!(hash, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn test_hash_file_blake3() {
        let file = write_temp_file(b"abc");
        let hash = hash_file(file.path(), &HashAlgorithm::Blake3).unwrap();
        // echo -n "abc" | b3sum
        assert_eq!(hash, "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");
    }

    #[test]
    fn test_hash_file_xxhash() {
        let file = write_temp_file(b"abc");
        let hash = hash_file(file.path(), &HashAlgorithm::Xxhash).unwrap();
        // echo -n "abc" | xxhsum -H3
        assert_eq!(hash, "78af5f94892f3950");
    }

    #[test]
    fn test_hash_file_empty() {
        let file = write_temp_file(b"");
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "" | sha256sum
        assert_eq!(sha256_hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");

        let blake3_hash = hash_file(file.path(), &HashAlgorithm::Blake3).unwrap();
        // echo -n "" | b3sum
        assert_eq!(blake3_hash, "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262");

        let xxhash_hash = hash_file(file.path(), &HashAlgorithm::Xxhash).unwrap();
        // echo -n "" | xxhsum -H3
        assert_eq!(xxhash_hash, "2d06800538d394c2");
    }

    #[test]
    fn test_hash_file_file_not_found() {
        let res = hash_file(std::path::Path::new("/nonexistent/file.txt"), &HashAlgorithm::Sha256);
        assert!(res.is_err());
    }

    #[test]
    fn test_hash_file_large() {
        let data = vec![b'a'; 100_000];
        let file = write_temp_file(&data);
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "$(printf 'a%.0s' {1..100000})" | sha256sum
        assert_eq!(sha256_hash, "6d1cf22d7cc09b085dfc25ee1a1f3ae0265804c607bc2074ad253bcc82fd81ee");
    }

    #[test]
    fn test_hash_file_binary() {
        let data = [0, 159, 146, 150, 255, 0, 100, 200];
        let file = write_temp_file(&data);
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -ne '\x00\x9f\x92\x96\xff\x00\x64\xc8' | sha256sum
        assert_eq!(sha256_hash, "a9775ff29a64b91e1913eff707fd599ad2e4a8146af0ad2adf6b890482614b28");
    }

    #[test]
    fn test_hash_file_repeated_pattern() {
        let data = b"abcabcabcabcabcabcabcabcabcabc";
        let file = write_temp_file(data);
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "abcabcabcabcabcabcabcabcabcabc" | sha256sum
        assert_eq!(sha256_hash, "ebae678bcd141cc9b9d532b264f854096b645af41490c787218299e1bbc6510e");
    }

    #[test]
    fn test_hash_file_unicode() {
        let data = "你好，世界! 🌍".as_bytes();
        let file = write_temp_file(data);
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "你好，世界! 🌍" | sha256sum
        assert_eq!(sha256_hash, "89880199154081b785c140d29675c9a1cf0123ecacfa0f2ce4a268d0d47e0dc9");
    }

    #[test]
    fn test_hash_file_whitespace() {
        let data = b"    \n\t  ";
        let file = write_temp_file(data);
        let sha256_hash = hash_file(file.path(), &HashAlgorithm::Sha256).unwrap();
        // echo -n "    \n\t  " | sha256sum
        assert_eq!(sha256_hash, "89ce52a490e4aefd754db143220612b0065cd16b9ae8f5512c266788336b2ab9");
    }
}


