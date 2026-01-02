// generate manifest for the sprites result

use crate::sprite::utils;
use crate::sprite::types;

pub fn generate_sprite_manifest(
    scan_result: &types::KeyframeScanResult,
    rows: u32,
    cols: u32,
    thumb_width: u32,
    thumb_height: u32,
) -> Vec<types::SpriteInfo> {
    let frames_per_sheet = (rows * cols) as usize;
    let mut manifest = Vec::new();

    for (i, frame) in scan_result.keyframes.iter().enumerate() {
        // Calculate which sheet it belongs to
        let sheet_index = (i / frames_per_sheet) + 1;
        
        // Calculate position inside that specific sheet
        let index_in_sheet = (i % frames_per_sheet) as u32;
        let x = (index_in_sheet % cols) * thumb_width;
        let y = (index_in_sheet / cols) * thumb_height;

        // Determine the "end time" (the start time of the NEXT keyframe)
        // If it's the last frame, we just add a small duration or leave it
        let next_time = scan_result.keyframes.get(i + 1)
            .map(|f| f.time_seconds)
            .unwrap_or(frame.time_seconds + 2.0); // Fallback for last frame

        manifest.push(types::SpriteInfo {
            time_start: frame.time_seconds,
            time_end: next_time,
            time_display: format!("{} --> {}", 
                frame.time_human, 
                utils::format_human_time(next_time)
            ),
            sheet_file: format!("sprite_sheet_{}.jpg", sheet_index),
            xywh: format!("{},{},{},{}", x, y, thumb_width, thumb_height),
        });
    }

    manifest
}