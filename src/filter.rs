use std::fs;
use std::path::Path;

pub fn can_be_filtered(file1: &Path, file2: &Path) -> bool {
    if !file1.exists() || !file2.exists() {
        eprintln!("One of the files does not exist.");
        return false;
    }

    let meta1 = match fs::metadata(file1) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let meta2 = match fs::metadata(file2) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if !meta1.is_file() || !meta2.is_file() {
        eprintln!("One or both paths are not regular files.");
        return false;
    }

    if meta1.len() != meta2.len() {
        println!("File sizes are different — skipping hash.");
        return false;
    }

    true
}
