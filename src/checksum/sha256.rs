use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub fn sha256_file_hash(file_path: String) -> io::Result<String> {
    const CHUNK_SIZE: u64 = 5 * 1024 * 1024; // 5MB

    let path = Path::new(&file_path);  // Convert String to Path
    let mut file = File::open(path)?;  // Open the file using the Path
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let mut hasher = Sha256::new();

    // 1. Read first chunkSize bytes (or the whole file if it's smaller)
    let first_chunk_to_read = std::cmp::min(file_size, CHUNK_SIZE);
    let mut take_handle = (&file).take(first_chunk_to_read);
    io::copy(&mut take_handle, &mut hasher)?;

    // 2. Logic for the rest of the file
    if file_size > 2 * CHUNK_SIZE {
        // Seek to the start of the last chunk
        file.seek(SeekFrom::Start(file_size - CHUNK_SIZE))?;
        let mut last_chunk_handle = (&file).take(CHUNK_SIZE);
        io::copy(&mut last_chunk_handle, &mut hasher)?;
    } else if file_size > CHUNK_SIZE {
        // If file is between 5MB and 10MB, read the remaining bytes
        // Position is already at CHUNK_SIZE after the first read
        io::copy(&mut file, &mut hasher)?;
    }

    // 3. Finalize hash and convert to hex string
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
