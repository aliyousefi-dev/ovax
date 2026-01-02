use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct VideoInfo {
    pub path: String,
    pub duration: u64,
    pub duration_human: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub codec: String,
}