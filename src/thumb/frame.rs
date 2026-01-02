
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;
use std::path::Path;

use crate::thumb::savejpg;

/// Extracts a frame from the middle of the video and saves it.
pub fn extract_middle_frame(video_path: &Path, output_path: &Path) -> Result<(), String> {
    if let Ok(mut ictx) = input(&video_path) {
        let middle_timestamp = ictx.duration() / 2;

        let input = ictx.streams().best(Type::Video).ok_or("No video stream found")?;
        let video_stream_index = input.index();
        
        let mut decoder = ffmpeg_next::codec::context::Context::from_parameters(input.parameters())
            .map_err(|e| e.to_string())?
            .decoder()
            .video()
            .map_err(|e| e.to_string())?;

        let _ = ictx.seek(middle_timestamp, ..middle_timestamp);

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::LANCZOS, 
        ).map_err(|e| e.to_string())?;

        let mut frame = Video::empty();
        let mut rgb_frame = Video::new(Pixel::RGB24, decoder.width(), decoder.height());

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                let _ = decoder.send_packet(&packet);
                if decoder.receive_frame(&mut frame).is_ok() {
                    scaler.run(&frame, &mut rgb_frame).map_err(|e| e.to_string())?;
                    
                    savejpg::save_as_jpg(&rgb_frame, output_path.to_str().ok_or("Invalid output path")?);
                    return Ok(());
                }
            }
        }
    }
    Err("Failed to extract frame".to_string())
}
