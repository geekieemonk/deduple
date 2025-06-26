use std::path::Path;
use img_hash::{HasherConfig, HashAlg};

pub fn are_images_similar(
    img1_path: &Path,
    img2_path: &Path,
    threshold: u32,
) -> Result<bool, String> {
   
    let img1 = image::open(img1_path)
        .map_err(|e| format!("Failed to open {}: {}", img1_path.display(), e))?;
   
    let img2 = image::open(img2_path)
        .map_err(|e| format!("Failed to open {}: {}", img2_path.display(), e))?;
    
    let hasher = HasherConfig::new()
        .hash_size(16, 16)
        .hash_alg(HashAlg::Gradient)
        .to_hasher();

   
    let hash1 = hasher.hash_image(&img1);
    let hash2 = hasher.hash_image(&img2);

   
    let dist = hash1.dist(&hash2);

    
    println!(
        "🔍 Comparing:\n  {} ↔ {}\n  → Distance = {}",
        img1_path.display(),
        img2_path.display(),
        dist
    );

    
    Ok(dist <= threshold)
}
