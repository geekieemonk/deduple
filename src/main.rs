
mod cli;
mod hash;
mod quarantine;
//mod filter; 
mod report;

use cli::CliArgs;
use clap::Parser;
use hash::compare_files;
use quarantine::quarantine_file;
use report::generate_report;

//use filter::can_be_filtered;
//use std::path::PathBuf

fn main() {
    let args = CliArgs::parse();
    let file1 = args.file1;
    let file2 = args.file2;
    let algorithm = args.algorithm;
    let dry_run = args.dry_run;
    let report_path = args.report;

    

    match compare_files(&file1, &file2, &algorithm) {
        Ok(true) => {
            println!("✅ Files are duplicates.");
            let quarantined = match quarantine_file(&file2, dry_run) {
                Ok(q) => q,
                Err(e) => {
                    eprintln!("Failed to quarantine file: {}", e);
                    None
                }
            };
            generate_report(&file1, &file2, true, &algorithm, quarantined, &report_path);
        }
        Ok(false) => {
            println!("❌ Files are not duplicates.");
            generate_report(&file1, &file2, false, &algorithm, None, &report_path);
        }
        Err(e) => {
            eprintln!("Error comparing files: {}", e);
        }
    }
}