mod cli;
mod hash;
mod quarantine;
mod report;
mod image_dedupe;
mod image_hash;

use cli::CliArgs;
use clap::Parser;
use hash::hash_file;
use quarantine::quarantine_file;
use report::generate_folder_report;
use image_dedupe::dedupe_images_in_folder;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::fs;
use quarantine::restore_file_from_quarantine;

#[derive(Serialize, Deserialize, Default, Clone)]
struct FileCacheEntry {
    mtime: i64,
    hash: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_cache_entry_serde() {
        let entry = FileCacheEntry { mtime: 123, hash: "abc".to_string() };
        let json = serde_json::to_string(&entry).unwrap();
        let de: FileCacheEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(de.mtime, 123);
        assert_eq!(de.hash, "abc");
    }
}


fn main() {
    let args = CliArgs::parse();

    // Handle restore option
    if let Some(ref restore_path) = args.restore {
        match restore_file_from_quarantine(restore_path) {
            Ok(true) => println!("Restoration successful."),
            Ok(false) => println!("Restoration failed or file not found in quarantine log."),
            Err(e) => eprintln!("Error during restoration: {}", e),
        }
        return;
    }

    if let Some(folder) = args.img_folder.as_ref() {
        if let Err(e) = dedupe_images_in_folder(folder, 10) {
            eprintln!("Image folder deduplication failed: {}", e);
        }
        return;
    }

    let algorithm = args.algorithm;
    let dry_run = args.dry_run;
    let report_path = args.report;
    let dir = args.dir;

    println!("Scanning folder: {}", dir.display());

    // Load cache
    let cache_path = "deduple_cache.json";
    let cache: std::collections::HashMap<String, FileCacheEntry> = if let Ok(file) = fs::File::open(cache_path) {
        serde_json::from_reader(file).unwrap_or_default()
    } else {
        std::collections::HashMap::new()
    };
    let mut files = Vec::new();
    for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_path_buf());
        }
    }

    println!("🔍 Files found: {}", files.len());

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut new_cache = cache.clone();
    for file in &files {
        let meta = match fs::metadata(file) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let mtime = meta.modified().ok().and_then(|t| t.elapsed().ok()).map(|e| e.as_secs() as i64).unwrap_or(0);
        let file_str = file.display().to_string();
        let use_cache = cache.get(&file_str).map_or(false, |entry| entry.mtime == mtime);
        let hash = if use_cache {
            cache.get(&file_str).unwrap().hash.clone()
        } else {
            match hash_file(file, &algorithm) {
                Ok(h) => {
                    new_cache.insert(file_str.clone(), FileCacheEntry { mtime, hash: h.clone() });
                    h
                },
                Err(e) => {
                    eprintln!("Failed to hash {}: {}", file.display(), e);
                    continue;
                }
            }
        };
        hash_map.entry(hash).or_default().push(file.clone());
    }
    // Save updated cache
    let _ = fs::write(cache_path, serde_json::to_string_pretty(&new_cache).unwrap());

    let mut duplicate_groups = Vec::new();
    for (_hash, group) in &hash_map {
        if group.len() > 1 {
            duplicate_groups.push(group.clone());
        }
    }

    println!("Found {} groups of duplicates.", duplicate_groups.len());

    let mut report_groups = Vec::new();
    let mut total_space_saved = 0u64;
    for (i, group) in duplicate_groups.iter().enumerate() {
        println!("\nGroup {}:", i + 1);
        for file in group {
            println!("  - {}", file.display());
        }
        let mut quarantined = Vec::new();
        // Space analysis: sum sizes of all but one file
        let mut group_space = 0u64;
        for file in group.iter().skip(1) {
            if let Ok(meta) = fs::metadata(file) {
                group_space += meta.len();
            }
            match quarantine_file(file, dry_run) {
                Ok(Some(q)) => {
                    println!(" Quarantined: {}", q.display());
                    quarantined.push(q);
                }
                Ok(None) => {} // dry-run or skipped
                Err(e) => eprintln!("Failed to quarantine {}: {}", file.display(), e),
            }
        }
        total_space_saved += group_space;
        report_groups.push((group.clone(), quarantined));
    }

    println!("\nPotential space saved: {:.2} MB", total_space_saved as f64 / 1_048_576.0);
    generate_folder_report(&algorithm, report_groups, &report_path);
    println!("Report saved to {}", report_path.display());
}
