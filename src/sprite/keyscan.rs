
use std::path::Path;
use ffmpeg_next::{format::input, media::Type};
use crate::sprite::types::KeyframeInfo;
use crate::sprite::types::KeyframeScanResult;

use crate::sprite::utils;

/// Finds all I-Frames (keyframes) in a given video file.
pub fn find_keyframes(video_path: &Path) -> Result<KeyframeScanResult, String> {
    // FIXED: Made `ictx` mutable so `ictx.packets()` can borrow it mutably.
    let mut ictx = input(video_path).map_err(|e| e.to_string())?;

    let input_stream = ictx
        .streams()
        .best(Type::Video)
        .ok_or_else(|| "Could not find a video stream".to_string())?;
    
    let video_index = input_stream.index();
    let time_base = input_stream.time_base();

    let mut keyframe_list: Vec<KeyframeInfo> = Vec::new();

    // Iterate through all packets in the file.
    for (stream, packet) in ictx.packets() {
        // Process only packets from our video stream.
        if stream.index() == video_index {
            // Check if the `KEY` flag is set on the packet.
            if packet.is_key() {
                if let Some(pts) = packet.pts() {
                    // Convert the packet's timestamp (PTS) into seconds.
                    let time_seconds = pts as f64 * f64::from(time_base);
                    let rounded_seconds = (time_seconds * 1000.0).round() / 1000.0;

                    keyframe_list.push(KeyframeInfo {
                        time_seconds: rounded_seconds,
                        time_human: utils::format_human_time(time_seconds),
                    });
                }
            }
        }
    }

    let result = KeyframeScanResult {
        total_keyframes: keyframe_list.len(),
        keyframes: keyframe_list,
    };

    Ok(result)
}
