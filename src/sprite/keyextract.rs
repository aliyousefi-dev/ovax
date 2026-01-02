
use std::path::{Path, PathBuf};
use std::fs;
use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::util::frame::video::Video;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use image::{Rgb};

/// Phase 1: Extracts only I-frames (Keyframes) from a video and saves them to a temp folder.
pub fn extract_keyframes_to_disk(
    video_path: &Path,
    temp_dir: &Path,
    thumb_width: u32,
    thumb_height: u32,
) -> Result<Vec<PathBuf>, String> {
    fs::create_dir_all(temp_dir).map_err(|e| format!("Dir error: {}", e))?;

    let mut ictx = input(&video_path).map_err(|e| e.to_string())?;
    let input_stream = ictx.streams().best(Type::Video).ok_or("No video stream")?;
    let video_index = input_stream.index();
    
    let context = ffmpeg_next::codec::context::Context::from_parameters(input_stream.parameters())
        .map_err(|e| e.to_string())?;
    let mut decoder = context.decoder().video().map_err(|e| e.to_string())?;
    
    // Scaler configured to resize to thumb_width/height
    let mut scaler = Context::get(
        decoder.format(), decoder.width(), decoder.height(),
        Pixel::RGB24, thumb_width, thumb_height, Flags::BICUBIC
    ).map_err(|e| e.to_string())?;

    let mut saved_paths = Vec::new();
    let mut decoded_frame = Video::empty();
    let mut rgb_frame = Video::empty();
    let mut frame_count = 0;

    // 1. Primary Loop
    for (stream, packet) in ictx.packets() {
        if stream.index() == video_index && packet.is_key() {
            if decoder.send_packet(&packet).is_ok() {
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    scaler.run(&decoded_frame, &mut rgb_frame).map_err(|e| e.to_string())?;
                    
                    let path = temp_dir.join(format!("frame_{:05}.png", frame_count));
                    
                    // FIXED: Use thumb_width/height here
                    image::ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(thumb_width, thumb_height, rgb_frame.data(0))
                        .ok_or("Buffer error")?
                        .save(&path)
                        .map_err(|e| e.to_string())?;
                    
                    saved_paths.push(path);
                    frame_count += 1;
                }
            }
        }
    }

    // 2. Flush the Decoder
    let _ = decoder.send_eof(); 
    while decoder.receive_frame(&mut decoded_frame).is_ok() {
        // Only save if it's actually a keyframe (I-Frame)
        if decoded_frame.is_key() {
            scaler.run(&decoded_frame, &mut rgb_frame).map_err(|e| e.to_string())?;
            
            let path = temp_dir.join(format!("frame_{:05}.png", frame_count));
            
            // FIXED: Changed decoder.width() -> thumb_width
            // FIXED: Changed decoder.height() -> thumb_height
            image::ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(thumb_width, thumb_height, rgb_frame.data(0))
                .ok_or("Buffer error")?
                .save(&path)
                .map_err(|e| e.to_string())?;
            
            saved_paths.push(path);
            frame_count += 1;
        }
    }

    Ok(saved_paths)
}