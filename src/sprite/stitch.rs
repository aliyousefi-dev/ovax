
use std::path::{Path, PathBuf};
use image::{ImageBuffer, Rgb};

/// Phase 2: Takes a list of paths and stitches them into sprite sheets.
pub fn stitch_frames_into_sprites(
    frame_paths: &[PathBuf],
    output_dir: &Path,
    rows: u32,
    cols: u32,
    thumb_width: u32,
    thumb_height: u32,
) -> Result<(), String> {
    let frames_per_sheet = (rows * cols) as usize;
    let sheet_width = cols * thumb_width;
    let sheet_height = rows * thumb_height;

    for (sheet_index, chunk) in frame_paths.chunks(frames_per_sheet).enumerate() {
        let mut sprite_sheet = ImageBuffer::<Rgb<u8>, _>::new(sheet_width, sheet_height);

        for (i, path) in chunk.iter().enumerate() {
            let img = image::open(path)
                .map_err(|e| format!("Failed to open {}: {}", path.display(), e))?
                .to_rgb8();

            let x = (i as u32 % cols) * thumb_width;
            let y = (i as u32 / cols) * thumb_height;

            image::imageops::replace(&mut sprite_sheet, &img, x.into(), y.into());
        }

        let out_path = output_dir.join(format!("sprite_sheet_{}.jpg", sheet_index + 1));
        sprite_sheet.save(&out_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
