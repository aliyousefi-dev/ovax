use std::path::Path;
use rayon::prelude::*;
use super::types::ExtractionResults;
use super::extractor::extract_middle_frame;

pub fn extract_multiple_frames(video_paths: Vec<String>, output_folder: &str) -> ExtractionResults {
    let output_dir = Path::new(output_folder);

    video_paths
        .par_iter()
        .map(|v_path| {
            let video_path = Path::new(v_path);
            
            // Create an output filename: e.g., video.mp4 -> video.jpg
            let file_stem = video_path.file_stem().unwrap_or_default().to_str().unwrap_or("thumb");
            let thumb_path = output_dir.join(format!("{}.jpg", file_stem));

            match extract_middle_frame(video_path, &thumb_path) {
                Ok(_) => Ok(v_path.clone()),
                Err(e) => Err((v_path.clone(), e)),
            }
        })
        .fold(ExtractionResults::default, |mut acc, res| {
            match res {
                Ok(path) => acc.successes.push(path),
                Err((path, err)) => { acc.failures.insert(path, err); }
            }
            acc
        })
        .reduce(ExtractionResults::default, |mut a, b| {
            a.successes.extend(b.successes);
            a.failures.extend(b.failures);
            a
        })
}