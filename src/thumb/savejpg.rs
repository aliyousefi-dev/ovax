
use ffmpeg_next::util::frame::video::Video;

/// Helper function to handle RGB padding and save via image crate.
pub(crate) fn save_as_jpg(frame: &Video, filename: &str) {
    let width = frame.width() as usize;
    let height = frame.height() as usize;
    let stride = frame.stride(0);
    let data = frame.data(0);

    let mut packed_data = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        let start = y * stride;
        let end = start + (width * 3);
        packed_data.extend_from_slice(&data[start..end]);
    }

    image::save_buffer(
        filename,
        &packed_data,
        width as u32,
        height as u32,
        image::ExtendedColorType::Rgb8,
    ).expect("Failed to save JPEG");
}