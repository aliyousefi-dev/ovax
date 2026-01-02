
use serde::Serialize;

/// Holds the information for a single keyframe.
#[derive(Serialize)]
pub struct KeyframeInfo {
    pub time_seconds: f64,
    pub time_human: String,
}

/// The final JSON structure that will be returned.
#[derive(Serialize)]
pub struct KeyframeScanResult {
    pub total_keyframes: usize,
    pub keyframes: Vec<KeyframeInfo>,
}

#[derive(Serialize)]
pub struct SpriteInfo {
    pub time_start: f64,
    pub time_end: f64,
    pub time_display: String, // e.g. "00:00:02.280 --> 00:00:03.680"
    pub sheet_file: String,   // e.g. "sprite_sheet_1.jpg"
    pub xywh: String,         // e.g. "640,0,160,90"
}