use walkdir::WalkDir;


/// Scans the target directory and returns a list of all .mp4 file paths.
pub fn scanvideos(target_dir: &str) -> Vec<String> {
    WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "mp4")) // Only mp4 files
        .map(|e| {
            let path = e.into_path().to_string_lossy().to_string();
            // Replace backslashes with forward slashes to match Linux file paths
            path.replace("\\", "/")
        })
        .collect()
}