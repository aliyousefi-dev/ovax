use rayon::prelude::*;
use crate::checksum::sha256::sha256_file_hash;
use crate::checksum::types::HashResults; // Import the struct from types.rs

pub fn sha256_multiple_file_hashes(file_paths: Vec<String>) -> HashResults {
    file_paths
        .par_iter()
        .map(|file_path| {
            match sha256_file_hash(file_path.clone()) {
                Ok(hash) => Ok((hash, file_path.clone())),
                Err(e) => Err((file_path.clone(), e.to_string())),
            }
        })
        .fold(HashResults::default, |mut acc, res| {
            match res {
                Ok((hash, path)) => { acc.successes.insert(hash, path); }
                Err((path, err)) => { acc.failures.insert(path, err); }
            }
            acc
        })
        .reduce(HashResults::default, |mut a, b| {
            a.successes.extend(b.successes);
            a.failures.extend(b.failures);
            a
        })
}