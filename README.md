# Deduple - File Deduplication Tool

A fast and efficient command-line tool for detecting and managing duplicate files in directories. Supports multiple hash algorithms and image similarity detection.

## Features

- **Multiple Hash Algorithms**: SHA256, Blake3, and XXHash support
- **Image Deduplication**: Detect similar images using perceptual hashing
- **File Quarantine**: Safely move duplicate files to quarantine
- **Caching**: Intelligent caching to speed up repeated scans
- **Dry Run Mode**: Preview changes without making them
- **Detailed Reports**: JSON reports with space savings analysis
- **File Restoration**: Restore files from quarantine
- **Cross-platform**: Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd deduple

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

The binary will be available as `deduple` (or `deduple.exe` on Windows).

## Usage

### Basic File Deduplication

```bash
# Scan a directory for duplicates using SHA256 (default)
deduple --dir /path/to/your/folder

# Use a different hash algorithm
deduple --dir /path/to/your/folder --algorithm blake3
deduple --dir /path/to/your/folder --algorithm xxhash

# Dry run (preview without making changes)
deduple --dir /path/to/your/folder --dry-run

# Specify custom report location
deduple --dir /path/to/your/folder --report my_report.json
```

### Image Deduplication

```bash
# Detect similar images in a folder
deduple --img-folder /path/to/images
```

### File Restoration

```bash
# Restore a file from quarantine
deduple --restore /path/to/file
```

## Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--dir` | Directory to scan for duplicates | Required |
| `--algorithm` | Hash algorithm: sha256, blake3, xxhash | sha256 |
| `--dry-run` | Preview changes without making them | false |
| `--report` | Path for the JSON report | report.json |
| `--img-folder` | Folder to scan for similar images | None |
| `--restore` | Restore a file from quarantine | None |

## Hash Algorithms

- **SHA256**: Cryptographically secure, slower but most reliable
- **Blake3**: Fast cryptographic hash, good balance of speed and security
- **XXHash**: Extremely fast non-cryptographic hash, best for performance

## How It Works

1. **File Discovery**: Recursively scans the specified directory
2. **Caching**: Uses modification time to avoid re-hashing unchanged files
3. **Hashing**: Computes file hashes using the selected algorithm
4. **Duplicate Detection**: Groups files with identical hashes
5. **Quarantine**: Moves duplicate files to a quarantine directory
6. **Reporting**: Generates a detailed JSON report

## Image Similarity Detection

For image deduplication, the tool uses perceptual hashing to detect visually similar images, even if they have different formats, sizes, or slight modifications.

## Output Files

- **deduple_cache.json**: Cache file for faster subsequent runs
- **report.json**: Detailed report of duplicates found and space saved
- **quarantine/**: Directory containing quarantined duplicate files

## Example Output

```
Scanning folder: /path/to/folder
🔍 Files found: 1250
Found 15 groups of duplicates.

Group 1:
  - /path/to/folder/file1.txt
  - /path/to/folder/copy_of_file1.txt
 Quarantined: quarantine/copy_of_file1.txt

Group 2:
  - /path/to/folder/image1.jpg
  - /path/to/folder/image1_copy.jpg
 Quarantined: quarantine/image1_copy.jpg

Potential space saved: 45.67 MB
Report saved to report.json
```

## Report Format

The JSON report contains:
- Algorithm used
- Duplicate groups with file paths
- Quarantined files
- Space saved in bytes
- Total space saved

## Safety Features

- **Dry Run Mode**: Preview all changes before making them
- **Quarantine System**: Files are moved, not deleted
- **File Restoration**: Restore files from quarantine if needed
- **Error Handling**: Graceful handling of permission and I/O errors

## Performance Tips

- Use XXHash for fastest performance on large directories
- The tool caches hashes, so subsequent runs are much faster
- For image deduplication, consider using a lower similarity threshold for more aggressive detection

## Troubleshooting

### Permission Errors
Ensure you have read/write permissions for the target directory.

### Out of Memory
For very large directories, consider scanning subdirectories separately.

### Image Processing Errors
Some image formats may not be supported. Convert to common formats (JPEG, PNG) if needed.

## Development

### Running Tests

```bash
cargo test
```

### Building for Development

```bash
cargo build
```

### Code Structure

- `src/main.rs`: Main application logic
- `src/cli.rs`: Command-line interface
- `src/hash.rs`: File hashing functionality
- `src/image_hash.rs`: Image similarity detection
- `src/image_dedupe.rs`: Image deduplication logic
- `src/quarantine.rs`: File quarantine management
- `src/report.rs`: Report generation

## License

[Add your license information here]

## Contributing

[Add contribution guidelines here]

## Acknowledgments

- Uses the `img_hash` crate for perceptual image hashing
- Built with Rust for performance and safety
- CLI powered by `clap` for excellent user experience 
