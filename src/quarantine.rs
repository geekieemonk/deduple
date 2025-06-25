use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::Utc;
pub fn quarantine_file(path: &Path, dry_run: bool) -> io::Result<Option<PathBuf>> {
    let quarantine_dir = Path::new(".quarantine");

    if dry_run {
        println!("[Dry Run] Would quarantine: {}", path.display());
        return Ok(None);
    }
    if !quarantine_dir.exists() {
        fs::create_dir(quarantine_dir)?;
    }
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid file path")),
    };

    let new_path = quarantine_dir.join(format!("{}_{}", timestamp, file_name));
    fs::rename(path, &new_path)?;

    println!("Quarantined: {}", new_path.display());
    Ok(Some(new_path))
}
