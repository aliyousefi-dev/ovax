use std::path::PathBuf;
use walkdir::WalkDir;


/// Scans the target directory and returns a list of all .mp4 file paths.
pub fn scanvideos(target_dir: &str) -> Vec<PathBuf> {
    WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "mp4"))
        .map(|e| e.into_path())
        .collect()
}