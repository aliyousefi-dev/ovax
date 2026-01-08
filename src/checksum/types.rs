use std::collections::HashMap;
use serde::Serialize;

/// Structure to hold both successful hashes and failed attempts
#[derive(Serialize,Debug, Default)]
pub struct HashResults {
    /// Mapping of Hash -> FilePath
    pub successes: HashMap<String, String>, 
    /// Mapping of FilePath -> ErrorMessage
    pub failures: HashMap<String, String>,  
}