use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize)]
pub struct ExtractionResults {
    pub successes: Vec<String>,          // List of videos successfully thumbnailed
    pub failures: HashMap<String, String>, // VideoPath -> Error Message
}