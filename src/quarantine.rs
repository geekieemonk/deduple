use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarantineLogEntry {
    pub original_path: String,
    pub quarantine_path: String,
    pub timestamp: String,
}

fn append_quarantine_log(entry: &QuarantineLogEntry) {
    let log_path = Path::new("quarantine_log.json");
    let mut log: Vec<QuarantineLogEntry> = if log_path.exists() {
        let file = fs::File::open(log_path).ok();
        if let Some(file) = file {
            serde_json::from_reader(file).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    log.push(entry.clone());
    if let Ok(json) = serde_json::to_string_pretty(&log) {
        let _ = fs::write(log_path, json);
    }
}

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

    // Log the quarantine action
    let entry = QuarantineLogEntry {
        original_path: path.display().to_string(),
        quarantine_path: new_path.display().to_string(),
        timestamp,
    };
    append_quarantine_log(&entry);

    println!("Quarantined: {}", new_path.display());
    Ok(Some(new_path))
}

pub fn restore_file_from_quarantine(original_path: &Path) -> io::Result<bool> {
    let log_path = Path::new("quarantine_log.json");
    if !log_path.exists() {
        eprintln!("No quarantine log found.");
        return Ok(false);
    }
    let file = fs::File::open(log_path)?;
    let mut log: Vec<QuarantineLogEntry> = serde_json::from_reader(file).unwrap_or_default();
    if let Some(pos) = log.iter().position(|entry| entry.original_path == original_path.display().to_string()) {
        let entry = &log[pos];
        let quarantine_path = Path::new(&entry.quarantine_path);
        if quarantine_path.exists() {
            fs::rename(quarantine_path, original_path)?;
            println!("Restored: {} -> {}", entry.quarantine_path, entry.original_path);
            log.remove(pos);
            let _ = fs::write(log_path, serde_json::to_string_pretty(&log).unwrap());
            return Ok(true);
        } else {
            eprintln!("Quarantined file not found: {}", entry.quarantine_path);
        }
    } else {
        eprintln!("No log entry found for: {}", original_path.display());
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_quarantine_file_dry_run() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let res = quarantine_file(tmp.path(), true).unwrap();
        assert!(res.is_none());
    }
    #[test]
    fn test_quarantine_file_invalid_path() {
        let res = quarantine_file(Path::new("/nonexistent/file.txt"), false);
        assert!(res.is_err());
    }
}
