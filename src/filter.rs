use std::fs;
use std::path::Path;

pub fn can_be_filtered(file1: &Path, file2: &Path) -> Result<bool, std::io::Error> {
    if !file1.exists() || !file2.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "One of the files does not exist."));
    }

    let meta1 = fs::metadata(file1)?;
    let meta2 = fs::metadata(file2)?;

    if !meta1.is_file() || !meta2.is_file() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "One or both paths are not regular files."));
    }

    if meta1.len() != meta2.len() {
        return Ok(false);
    }

    Ok(true)
}
