use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "deduple", about = "Detect and quarantine duplicate files in a directory.")]
pub struct CliArgs {
    #[arg(long, value_enum, default_value = "sha256")]
    pub algorithm: HashAlgorithm,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    #[arg(long, default_value = "report.json")]
    pub report: PathBuf,
    #[arg(long)]
    pub dir: PathBuf,
       #[arg(long)]
    pub img_folder: Option<PathBuf>,

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
    Xxhash,
}
