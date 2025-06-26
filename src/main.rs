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

fn main() {
    let args = CliArgs::parse();

    
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

    println!("📂 Scanning folder: {}", dir.display());

    
    let mut files = Vec::new();
    for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_path_buf());
        }
    }

    println!("🔍 Files found: {}", files.len());

    let mut hash_map: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for file in &files {
        match hash_file(file, &algorithm) {
            Ok(hash) => {
                hash_map.entry(hash).or_default().push(file.clone());
            }
            Err(e) => {
                eprintln!("❌ Failed to hash {}: {}", file.display(), e);
            }
        }
    }

   
    let mut duplicate_groups = Vec::new();
    for (_hash, group) in &hash_map {
        if group.len() > 1 {
            duplicate_groups.push(group.clone());
        }
    }

    println!("📦 Found {} groups of duplicates.", duplicate_groups.len());

    let mut report_groups = Vec::new();

    for (i, group) in duplicate_groups.iter().enumerate() {
        println!("\nGroup {}:", i + 1);
        for file in group {
            println!("  - {}", file.display());
        }

        let mut quarantined = Vec::new();

       
        for file in group.iter().skip(1) {
            match quarantine_file(file, dry_run) {
                Ok(Some(q)) => {
                    println!("🧼 Quarantined: {}", q.display());
                    quarantined.push(q);
                }
                Ok(None) => {} // dry-run or skipped
                Err(e) => eprintln!("⚠️ Failed to quarantine {}: {}", file.display(), e),
            }
        }

        report_groups.push((group.clone(), quarantined));
    }

    
generate_folder_report(&algorithm, report_groups, &report_path);
println!("✅ Report saved to {}", report_path.display());

}
