use std::fs;
use std::path::{Path, PathBuf};
use crate::image_hash::are_images_similar;

pub fn dedupe_images_in_folder(folder: &Path, threshold: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut images: Vec<PathBuf> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();
                matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp")
            } else {
                false
            }
        })
        .collect();

    images.sort();

    for i in 0..images.len() {
        for j in (i + 1)..images.len() {
            let img1 = &images[i];
            let img2 = &images[j];

            match are_images_similar(img1, img2, threshold) {
                Ok(true) => println!("Similar: {} and {}", img1.display(), img2.display()),
                Ok(false) => {}
                Err(e) => eprintln!("Error comparing {} and {}: {}", img1.display(), img2.display(), e),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dedupe_images_in_folder_empty() {
        // Should succeed with empty/nonexistent folder
        let tmp = tempfile::tempdir().unwrap();
        let res = dedupe_images_in_folder(tmp.path(), 10);
        assert!(res.is_ok());
    }
}
