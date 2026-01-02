

use std::path::Path;
use ffmpeg_next::format::{input, output, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;
use ffmpeg_next::{Dictionary, Packet, Rational};
use ffmpeg_next::codec;

pub fn generate_preview(
    video_path: &Path,
    output_path: &Path,
    start_time: f64,
    duration: f64,
) -> Result<(), String> {
    // --- 1. SETUP INPUT ---
    let mut ictx = input(video_path).map_err(|e| e.to_string())?;
    let input_stream = ictx.streams().best(Type::Video).ok_or("No video stream found")?;
    let video_index = input_stream.index();
    let input_time_base = input_stream.time_base();

    let mut decoder = codec::context::Context::from_parameters(input_stream.parameters()).map_err(|e| e.to_string())?
        .decoder().video().map_err(|e| e.to_string())?;

    // --- 2. SETUP OUTPUT ---
    let mut octx = output(output_path).map_err(|e| e.to_string())?;
    let codec = codec::encoder::find(codec::Id::VP8).ok_or("VP8 codec not found")?;

    // --- 3. CONFIGURE ENCODER ---
    let frame_rate = 25.0;
    let mut ost = octx.add_stream(codec).map_err(|e| e.to_string())?;
    let ost_index = ost.index();
    
    let mut encoder = {
        let mut enc = codec::context::Context::new().encoder().video().map_err(|e| e.to_string())?;
        enc.set_width(320);
        enc.set_height((320 * decoder.height()) / decoder.width());
        enc.set_format(Pixel::YUV420P);
        
        enc.set_time_base(Rational::new(1, frame_rate as i32));

        let mut opts = Dictionary::new();
        opts.set("b", "500k");
        opts.set("quality", "realtime");
        opts.set("cpu-used", "7");
        opts.set("qmin", "2");
        opts.set("qmax", "31");
        enc.open_as_with(codec, opts).map_err(|e| e.to_string())?
    };
    ost.set_parameters(&encoder);

    // --- 4. SETUP SCALER AND WRITE HEADER ---
    let mut scaler = Context::get(
        decoder.format(), decoder.width(), decoder.height(),
        encoder.format(), encoder.width(), encoder.height(),
        Flags::FAST_BILINEAR,
    ).map_err(|e| e.to_string())?;
    
    octx.write_header().map_err(|e| e.to_string())?;
    
    // --- 5. SEEK AND PROCESS ---
    let seek_pts = (start_time / f64::from(input_time_base)) as i64;
    ictx.seek(seek_pts, ..seek_pts).map_err(|e| e.to_string())?;
    decoder.flush();

    let max_frames_to_encode = (duration * frame_rate).ceil() as i64;
    let mut frame = Video::empty();
    let mut scaled_frame = Video::empty();
    let mut out_packet = Packet::empty();
    let mut frame_count = 0i64;
    
    let mut packets = ictx.packets();

    'processing: while let Some((stream, pkt)) = packets.next() {
        if stream.index() == video_index {
            if decoder.send_packet(&pkt).is_ok() {
                while decoder.receive_frame(&mut frame).is_ok() {
                    
                    // --- THE REAL FIX: DISCARD FRAMES BEFORE START TIME ---
                    // After seeking, FFmpeg gives us frames from a nearby keyframe.
                    // We must manually check the timestamp of each *decoded frame* and skip
                    // any that are before our desired start_time.
                    if let Some(pts) = frame.pts() {
                        let current_frame_sec = pts as f64 * f64::from(input_time_base);
                        if current_frame_sec < start_time {
                            continue; // Skip this frame and keep decoding.
                        }
                    } else {
                        continue; // No timestamp, can't verify, so skip.
                    }
                    // --- END OF FIX ---

                    // Now that we're at the right time, check if we're done.
                    if frame_count >= max_frames_to_encode {
                        break 'processing;
                    }

                    scaler.run(&frame, &mut scaled_frame).map_err(|e| e.to_string())?;
                    
                    if encoder.send_frame(&scaled_frame).is_ok() {
                        while encoder.receive_packet(&mut out_packet).is_ok() {
                            out_packet.set_stream(ost_index);
                            
                            let pts = (frame_count as f64 / frame_rate * 1000.0).round() as i64;
                            out_packet.set_pts(Some(pts));
                            out_packet.set_dts(Some(pts));
                            
                            out_packet.write_interleaved(&mut octx).map_err(|e| e.to_string())?;
                        }
                    }
                    // Only increment the count for frames we actually process.
                    frame_count += 1;
                }
            }
        }
    }

    // --- 6. FLUSH ENCODER ---
    encoder.send_eof().map_err(|e| e.to_string())?;
    while encoder.receive_packet(&mut out_packet).is_ok() {
        out_packet.set_stream(ost_index);
        
        let pts = (frame_count as f64 / frame_rate * 1000.0).round() as i64;
        out_packet.set_pts(Some(pts));
        out_packet.set_dts(Some(pts));
        out_packet.write_interleaved(&mut octx).map_err(|e| e.to_string())?;
        frame_count += 1;
    }

    // --- 7. WRITE TRAILER ---
    octx.write_trailer().map_err(|e| e.to_string())?;
    Ok(())
}
