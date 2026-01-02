use rayon::prelude::*;
use walkdir::WalkDir;
use std::fs::File;

use crate::utils::{format_duration, format_size};
use crate::scan::types::VideoInfo;

pub fn collect_videos(target_dir: &str) -> Vec<VideoInfo> {
    let paths: Vec<_> = WalkDir::new(target_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext.to_string_lossy().eq_ignore_ascii_case("mp4")))
        .map(|e| e.path().to_owned())
        .collect();

    let results: Vec<VideoInfo> = paths
        .par_iter()
        .filter_map(|path| {
            let file = File::open(path).ok()?;
            let metadata = file.metadata().ok()?;
            let size = metadata.len();
            let reader = std::io::BufReader::new(file);

            if let Ok(m) = mp4::Mp4Reader::read_header(reader, size) {
                let duration_secs = m.duration().as_secs();
                let mut info = VideoInfo {
                    path: path.to_string_lossy().replace('\\', "/"),
                    duration: duration_secs,
                    duration_human: format_duration(duration_secs),
                    size_bytes: size,
                    size_human: format_size(size),
                    width: 0, height: 0, fps: 0.0,
                    codec: String::from("unknown"),
                };

                for track in m.tracks().values() {
                    if let Ok(mp4::TrackType::Video) = track.track_type() {
                        
                        info.width = track.width() as u32;
                        info.height = track.height() as u32;
                        
                        info.codec = track.box_type().ok()?.to_string();

                        // FINAL FIX: Access .stts directly as it's not an Option.
                        let stts = &track.trak.mdia.minf.stbl.stts;
                        if !stts.entries.is_empty() {
                            let timescale = track.timescale() as f64;
                            let delta = stts.entries[0].sample_delta as f64;
                            if delta > 0.0 { info.fps = (timescale / delta * 100.0).round() / 100.0; }
                        }
                        break;
                    }
                }
                return Some(info);
            }
            None
        })
        .collect();
        
    results
}
