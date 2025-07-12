use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::cli::HashAlgorithm;

#[derive(Serialize)]
pub struct DuplicateGroup {
    pub files: Vec<String>,
    pub quarantined: Vec<String>,
    pub space_saved_bytes: u64,
}

#[derive(Serialize)]
pub struct FolderReport {
    pub algorithm_used: String,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub total_space_saved_bytes: u64,
}

pub fn generate_folder_report(algorithm: &HashAlgorithm, groups: Vec<(Vec<PathBuf>, Vec<PathBuf>)>, output_path: &Path) {
    let mut total_space_saved = 0u64;
    let duplicate_groups: Vec<DuplicateGroup> = groups.into_iter().map(|(files, quarantined)| {
        let mut space_saved = 0u64;
        for file in quarantined.iter() {
            if let Ok(meta) = std::fs::metadata(file) {
                space_saved += meta.len();
            }
        }
        total_space_saved += space_saved;
        DuplicateGroup {
            files: files.iter().map(|p| p.display().to_string()).collect(),
            quarantined: quarantined.iter().map(|p| p.display().to_string()).collect(),
            space_saved_bytes: space_saved,
        }
    }).collect();
    let report = FolderReport {
        algorithm_used: format!("{:?}", algorithm),
        duplicate_groups,
        total_space_saved_bytes: total_space_saved,
    };
    match serde_json::to_string_pretty(&report) {
        Ok(json_str) => {
            if let Err(e) = File::create(output_path).and_then(|mut f| f.write_all(json_str.as_bytes())) {
                eprintln!("Failed to write report: {}", e);
            } else {
                println!("Report written to {}", output_path.display());
            }
        }
        Err(e) => eprintln!("Failed to serialize report: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::cli::HashAlgorithm;
    #[test]
    fn test_generate_folder_report_writes_file() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let groups = vec![(vec![PathBuf::from("a.txt")], vec![PathBuf::from("b.txt")])];
        generate_folder_report(&HashAlgorithm::Sha256, groups, tmp.path());
        let contents = std::fs::read_to_string(tmp.path()).unwrap();
        assert!(contents.contains("algorithm_used"));
    }
}
