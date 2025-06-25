use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::cli::HashAlgorithm;

#[derive(Serialize)]
pub struct Report {
    file1: String,
    file2: String,
    are_duplicates: bool,
    algorithm_used: String,
    quarantined: Option<String>,
}

pub fn generate_report(file1: &Path, file2: &Path, are_duplicates: bool, algo: &HashAlgorithm, quarantine_path: Option<PathBuf>, output_path: &Path) {
    let report = Report {
        file1: file1.display().to_string(),
        file2: file2.display().to_string(),
        are_duplicates,
        algorithm_used: format!("{:?}", algo),
        quarantined: quarantine_path.map(|p| p.display().to_string()),
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
