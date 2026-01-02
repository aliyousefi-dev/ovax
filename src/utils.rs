// src/utils.rs
use serde::Serialize;

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB { format!("{:.2} GB", bytes as f64 / GB as f64) }
    else if bytes >= MB { format!("{:.2} MB", bytes as f64 / MB as f64) }
    else if bytes >= KB { format!("{:.2} KB", bytes as f64 / KB as f64) }
    else { format!("{} B", bytes) }
}

pub fn print_json<T: Serialize>(data: &T, pretty: bool) {
    let result = if pretty {
        serde_json::to_string_pretty(data)
    } else {
        serde_json::to_string(data)
    };

    match result {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error serializing JSON: {}", e);
            std::process::exit(1);
        }
    }
}
