use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::cli::HashAlgorithm;

#[derive(Serialize)]
pub struct DuplicateGroup {
    pub files: Vec<String>,
    pub quarantined: Vec<String>,
}

#[derive(Serialize)]
pub struct FolderReport {
    pub algorithm_used: String,
    pub duplicate_groups: Vec<DuplicateGroup>,
}

pub fn generate_folder_report(algorithm: &HashAlgorithm, groups: Vec<(Vec<PathBuf>, Vec<PathBuf>)>, output_path: &Path) {
    let duplicate_groups: Vec<DuplicateGroup> = groups.into_iter().map(|(files, quarantined)| {
        DuplicateGroup {
            files: files.iter().map(|p| p.display().to_string()).collect(),
            quarantined: quarantined.iter().map(|p| p.display().to_string()).collect(),
        }
    }).collect();
    let report = FolderReport {
        algorithm_used: format!("{:?}", algorithm),
        duplicate_groups,
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
